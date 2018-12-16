FROM rust:1 AS builder
RUN groupadd -r app && useradd --no-log-init -r -g app app
WORKDIR /usr/src/healthcheck
COPY . .
RUN cargo build --release

FROM scratch
WORKDIR /usr/healthcheck
COPY --from=builder /usr/src/healthcheck/target/release/healthcheck .
COPY --from=builder /etc/passwd /etc/passwd
USER app
HEALTHCHECK --interval=30s --timeout=3s \
  CMD ./healthcheck
