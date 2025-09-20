# syntax=docker/dockerfile:1
FROM lukemathwalker/cargo-chef:latest-rust-1.78 AS chef
WORKDIR /build

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

# ---------- auth-svc ----------
FROM gcr.io/distroless/cc-debian12:nonroot AS auth
COPY --from=builder /build/target/release/auth-svc /usr/local/bin/auth-svc
ENTRYPOINT ["/usr/local/bin/auth-svc"]

# ---------- user-svc ----------
FROM gcr.io/distroless/cc-debian12:nonroot AS user
COPY --from=builder /build/target/release/user-svc /usr/local/bin/user-svc
ENTRYPOINT ["/usr/local/bin/user-svc"]

# ---------- migrator ----------
FROM gcr.io/distroless/cc-debian12:nonroot AS migrator
COPY --from=builder /build/target/release/migrator /usr/local/bin/migrator
ENTRYPOINT ["/usr/local/bin/migrator"]
