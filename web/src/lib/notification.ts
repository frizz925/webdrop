import type { ObjectID } from './models';

export interface NotificationEvent {
	name: string;
}

export interface ObjectEvent extends NotificationEvent {
	objectID: ObjectID;
}

export type NotificationHandler = (evt: NotificationEvent) => void;

export interface NotificationHandlers {
	[key: string]: NotificationHandler;
}
