FROM docker.io/paritytech/ci-unified:latest as builder

# Using a specific tag instead of latest is recommended for production
FROM docker.io/paritytech/ci-unified:1.70.0-bullseye as builder

WORKDIR /polkadot
COPY . /polkadot

# Optionally install system dependencies if needed
RUN apt-get update && apt-get install -y ...

RUN cargo fetch
RUN cargo build --locked --release

FROM docker.io/parity/base-bin:latest

# Verify binary name matches your project's output
COPY --from=builder /polkadot/target/release/healer-network-node /usr/local/bin

USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /polkadot polkadot && \
	mkdir -p /data /polkadot/.local/share/healer-network-node && \
	chown -R polkadot:polkadot /data /polkadot/.local && \
	ln -s /data /polkadot/.local/share/healer-network-node/chains && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/healer-network-node --version

USER polkadot

# Default ports for Substrate nodes
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

# Set default arguments to specify the correct data path
ENTRYPOINT ["/usr/local/bin/healer-network-node"]
CMD ["--base-path=/data", "--rpc-cors=all", "--unsafe-rpc-external", "--rpc-methods=unsafe", "--rpc-external"]
