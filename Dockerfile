FROM rust:1.38

WORKDIR /usr/src/stock
COPY . .

RUN apt update && \
    apt install -y cmake golang lynx && \
    wget https://github.com/protocolbuffers/protobuf/releases/download/v3.11.3/protoc-3.11.3-linux-x86_64.zip && \
    unzip protoc-3.11.3-linux-x86_64.zip -d protoc

ENV PATH="/usr/src/stock/protoc/bin:${PATH}"

RUN cargo install --path .

CMD ["server", "--addr", "0.0.0.0:9090"]
