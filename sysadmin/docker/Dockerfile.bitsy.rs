FROM ubuntu:22.04 AS builder

WORKDIR /app

COPY . .

RUN apt update && apt install -y \
    build-essential \
    curl \
    libpq-dev \
    software-properties-common \
    curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo build --release --all-targets


FROM ubuntu:22.04

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends \
    ca-certificates \
    libpq-dev \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y

COPY --from=builder /app/target/release/bitsy .
COPY --from=builder /app/target/release/bitsy.d .


EXPOSE ${PORT}

RUN chmod +x ./bitsy
CMD exec ./bitsy --host "${HOST}" --port "${PORT}" --pg-host "${PG_HOST}" --pg-port "${PG_PORT}" --pg-password "${PG_PASSWORD}"


