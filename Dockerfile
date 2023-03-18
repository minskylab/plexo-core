# Clonar el repositorio y el submódulo
FROM alpine/git as submodule
WORKDIR /tmp
RUN git clone --recursive https://github.com/minskylab/plexo-platform

# Instalar dependencias solo cuando sea necesario
FROM node:16-alpine AS platform-deps
# Revisa https://github.com/nodejs/docker-node/tree/b4117f9333da4138b03a546ec926ef50a31506c3#nodealpine para entender por qué se puede necesitar libc6-compat.
RUN apk add --no-cache libc6-compat
WORKDIR /app
COPY --from=submodule /tmp/plexo-platform/package.json* /tmp/plexo-platform/yarn.lock* ./
RUN yarn install --frozen-lockfile

# Reconstruir el código fuente solo cuando sea necesario
FROM node:16-alpine AS platform-builder
WORKDIR /app
COPY --from=platform-deps /app/node_modules ./node_modules
COPY --from=submodule /tmp/plexo-platform .
# Next.js recopila datos de telemetría completamente anónimos sobre el uso general.
# Obtén más información aquí: https://nextjs.org/telemetry
# Descomenta la siguiente línea en caso de que desees deshabilitar la telemetría durante la construcción.
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