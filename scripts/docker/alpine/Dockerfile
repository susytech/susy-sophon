FROM alpine:edge AS builder

# show backtraces
ENV RUST_BACKTRACE 1

RUN apk add --no-cache \
  build-base \
  cargo \
  cmake \
  eudev-dev \
  linux-headers \
  perl \
  rust

WORKDIR /susy
COPY . /susy
RUN cargo build --release --target x86_64-alpine-linux-musl --verbose
RUN strip target/x86_64-alpine-linux-musl/release/susy


FROM alpine:edge

# show backtraces
ENV RUST_BACKTRACE 1

RUN apk add --no-cache \
  libstdc++ \
  eudev-libs \
  libgcc

RUN addgroup -g 1000 susy \
  && adduser -u 1000 -G susy -s /bin/sh -D susy

USER susy

EXPOSE 8080 8545 8180

WORKDIR /home/susy

RUN mkdir -p /home/susy/.local/share/io.susy.sophon/
COPY --chown=susy:susy --from=builder /susy/target/x86_64-alpine-linux-musl/release/susy ./

ENTRYPOINT ["./susy"]
