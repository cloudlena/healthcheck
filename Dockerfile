FROM docker.io/library/rust:1 AS builder
RUN groupadd -r healthcheck && useradd --no-log-init -r -g healthcheck healthcheck
WORKDIR /usr/src/healthcheck
COPY . ./
RUN cargo build --release

FROM scratch
WORKDIR /usr/src/healthcheck/healthcheck
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /usr/src/healthcheck/target/release/healthcheck .
USER healthcheck
HEALTHCHECK --interval=30s --timeout=3s \
  CMD /usr/src/healthcheck/healthcheck
