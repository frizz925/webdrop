FROM rust:alpine AS server
WORKDIR /usr/src/webdrop
COPY . .
RUN apk add --no-cache musl-dev
RUN --mount=type=bind,src=./target,dst=/usr/src/webdrop/target,rw=true \
  --mount=type=cache,id=cargo,target=/usr/local/cargo/registry \
  cargo install --path .

FROM node:22-slim AS web
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY . /app
WORKDIR /app/web
RUN --mount=type=cache,id=pnpm,target=/pnpm/store \
  pnpm install --frozen-lockfile && pnpm run build

FROM alpine:edge
WORKDIR /app
COPY --from=server /usr/local/cargo/bin/webdrop /usr/local/bin/webdrop
COPY --from=web /app/web/build /app/web/build
EXPOSE 8000
CMD [ "webdrop" ]
