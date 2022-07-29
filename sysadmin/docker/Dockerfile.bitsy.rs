FROM ubuntu:22.04 AS builder

WORKDIR /app

COPY bitsy-rs/ .

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo build --release


FROM ubuntu:22.04

COPY --from=builder ./target/release/bitsy .

RUN chmod +x ./bitsy
CMD exec ./bitsy
