# Install dependencies only when needed
FROM node:16-alpine AS platform-deps
# Check https://github.com/nodejs/docker-node/tree/b4117f9333da4138b03a546ec926ef50a31506c3#nodealpine to understand why libc6-compat might be needed.
RUN apk add --no-cache libc6-compat
WORKDIR /app

# Install dependencies based on the preferred package manager
COPY plexo-platform/package.json plexo-platform/yarn.lock ./
RUN \
    if [ -f yarn.lock ]; then yarn --frozen-lockfile; \
    elif [ -f package-lock.json ]; then npm ci; \
    elif [ -f pnpm-lock.yaml ]; then yarn global add pnpm && pnpm i --frozen-lockfile; \
    else echo "Lockfile not found." && exit 1; \
    fi


# Rebuild the source code only when needed
FROM node:16-alpine AS platform-builder
WORKDIR /app
COPY --from=platform-deps /app/node_modules ./node_modules
COPY ./plexo-platform .

# Next.js collects completely anonymous telemetry data about general usage.
# Learn more here: https://nextjs.org/telemetry
# Uncomment the following line in case you want to disable telemetry during the build.
# ENV NEXT_TELEMETRY_DISABLED 1

RUN yarn build

# Start with a rust alpine image
FROM rust:1-alpine3.16 as core-builder
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
# if needed, add additional dependencies here
RUN apk add --no-cache musl-dev
# RUN apk add --no-cache pkgconfig
RUN apk add --no-cache libressl-dev

# set the workdir and copy the source into it
WORKDIR /app
COPY ./ /app
# do a release build
RUN cargo build --release
RUN strip target/release/plexo

# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.16 as core
# if needed, install additional dependencies here
RUN apk add --no-cache libgcc
RUN apk add --no-cache libressl-dev

COPY --from=platform-builder /app/out ./plexo-platform/out
# copy the binary into the final image
COPY --from=core-builder /app/target/release/plexo .
# set the binary as entrypoint
ENTRYPOINT ["/plexo"]