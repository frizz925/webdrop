use std::{
    collections::{HashMap, VecDeque},
    ops::DerefMut,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex, RwLock,
    },
};

use tokio::sync::Notify;

pub type ChannelId = usize;

struct InnerPubSub<T> {
    counter: ChannelId,
    channels: HashMap<ChannelId, Arc<Subscriber<T>>>,
}

impl<T> InnerPubSub<T> {
    fn new() -> Self {
        Self {
            counter: 0,
            channels: HashMap::new(),
        }
    }
}

pub struct Subscriber<T> {
    pubsub: Arc<RwLock<InnerPubSub<T>>>,
    id: ChannelId,
    buf: Mutex<VecDeque<T>>,
    notify: Notify,
    backlog: usize,
    counter: AtomicUsize,
}

impl<T> Subscriber<T> {
    fn new(pubsub: Arc<RwLock<InnerPubSub<T>>>, id: ChannelId, backlog: usize) -> Self {
        Self {
            pubsub,
            id,
            buf: Mutex::new(VecDeque::with_capacity(backlog)),
            notify: Notify::new(),
            backlog,
            counter: AtomicUsize::new(0),
        }
    }

    fn push(&self, value: T) {
        let mut buf = self.buf.lock().unwrap();
        if self.counter.load(Ordering::Relaxed) >= self.backlog {
            buf.pop_front();
            buf.push_back(value);
        } else {
            buf.push_back(value);
            self.counter.fetch_add(1, Ordering::Relaxed);
        }
        self.notify.notify_one();
    }

    pub async fn pop(&self) -> VecDeque<T> {
        if self.counter.load(Ordering::Relaxed) <= 0 {
            self.notify.notified().await;
        }
        let mut cur = self.buf.lock().unwrap();
        let mut buf = VecDeque::with_capacity(self.backlog);
        std::mem::swap(&mut buf, cur.deref_mut());
        self.counter.store(0, Ordering::Relaxed);
        buf
    }
}

impl<T> Drop for Subscriber<T> {
    fn drop(&mut self) {
        let mut dispatcher = self.pubsub.write().unwrap();
        dispatcher.channels.remove(&self.id);
    }
}

pub struct PubSub<T> {
    inner: Arc<RwLock<InnerPubSub<T>>>,
    backlog: usize,
}

impl<T: Clone + ?Sized> PubSub<T> {
    pub fn new(backlog: usize) -> Self {
        Self {
            inner: Arc::new(RwLock::new(InnerPubSub::new())),
            backlog,
        }
    }

    pub fn subscribe(&self) -> Arc<Subscriber<T>> {
        let mut inner = self.inner.write().unwrap();
        let id = inner.counter;
        let ch = Arc::new(Subscriber::new(self.inner.clone(), id, self.backlog));
        inner.channels.insert(id, ch.clone());
        inner.counter += 1;
        ch
    }

    pub fn publish(&self, value: &T) {
        let inner = self.inner.read().unwrap();
        for ch in inner.channels.values() {
            ch.push(value.to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::task::JoinError;

    use super::PubSub;

    #[tokio::test]
    async fn test_dispatcher() -> Result<(), JoinError> {
        let dispatcher = PubSub::new(1);
        let unexpected = "Should not be received".to_owned();
        dispatcher.publish(&unexpected);

        let channel = dispatcher.subscribe();
        let handle = tokio::spawn(async move {
            let mut values = channel.pop().await;
            values.pop_front().unwrap()
        });

        let expected = "Hello dispatcher".to_owned();
        dispatcher.publish(&expected);
        let result = handle.await?;
        assert_eq!(result, expected);

        Ok(())
    }
}
