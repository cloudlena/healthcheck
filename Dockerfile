FROM rust:1 AS builder
RUN groupadd -r app && useradd --no-log-init -r -g app app
WORKDIR /app
COPY . ./
RUN cargo build --release

FROM scratch
WORKDIR /healthcheck
COPY --from=builder /app/target/release/healthcheck .
COPY --from=builder /etc/passwd /etc/passwd
USER app
HEALTHCHECK --interval=30s --timeout=3s \
  CMD ./healthcheck
