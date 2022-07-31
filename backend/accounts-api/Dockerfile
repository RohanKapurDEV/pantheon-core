FROM ubuntu:20.04 as build

ARG DATABASE_URL=${DATABASE_URL}
ENV DATABASE_URL=${DATABASE_URL}

ARG CRANK_AUTHORITY=${CRANK_AUTHORITY}
ENV CRANK_AUTHORITY=${CRANK_AUTHORITY}

ARG MAX_CONNECTIONS=${MAX_CONNECTIONS}
ENV MAX_CONNECTIONS=${MAX_CONNECTIONS}

RUN apt-get update
RUN apt-get install --no-install-recommends -y \
    ca-certificates curl build-essential pkg-config libssl-dev libpq-dev libudev-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

# Change to COPY ./.. . when switching to Docker compose syntax for multiple builds
COPY . .

RUN cargo build --release

FROM ubuntu:20.04 as run

RUN apt-get update
RUN apt-get install --no-install-recommends -y curl ca-certificates libssl-dev libpq-dev libudev-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY --from=build /usr/src/app/target/release/accounts-api /usr/local/bin/accounts-api

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/accounts-api"]