# Stage 1: build
FROM rust:alpine AS builder

# 中国大陆推荐配置
# RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories
RUN apk add --no-cache musl-dev

WORKDIR /usr/src/rscms

# 中国大陆推荐配置
# RUN mkdir -vp ${CARGO_HOME:-/root/.cargo} && \
#     tee -a ${CARGO_HOME:-/root/.cargo}/config.toml <<EOF
#     [source.crates-io]
#     replace-with = 'ustc'
    
#     [source.ustc]
#     registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
# EOF
# RUN echo "The config.toml is:"
# RUN cat ${CARGO_HOME:-/root/.cargo}/config.toml

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl


# Stage 2: copy the executable to a minimal image
FROM alpine:latest

WORKDIR /bin

COPY --from=builder /usr/src/rscms/target/x86_64-unknown-linux-musl/release/rscms .

CMD ["/bin/rscms"]
