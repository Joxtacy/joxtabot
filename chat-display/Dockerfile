# use the official Bun image
# see all versions at https://hub.docker.com/r/oven/bun/tags
FROM oven/bun:1 as base
WORKDIR /usr/src/app

FROM base AS release
COPY . .
RUN bun install --frozen-lockfile

# run the app
# Using `USER bun` does not work with `docker compose watch`
# USER bun
EXPOSE 3000/tcp
ENTRYPOINT [ "bun", "run", "dev" ]

