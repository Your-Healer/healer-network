FROM docker.io/paritytech/ci-unified:latest as builder

# Using a specific tag instead of latest is recommended for production
# FROM docker.io/paritytech/ci-unified:bullseye-1.85.0-2025-01-28 as builder

WORKDIR /healer-network
COPY . /healer-network

# Optionally install system dependencies if needed
RUN apt-get update && apt-get install -y

RUN cargo fetch
RUN cargo build --workspace --locked --release

FROM docker.io/parity/base-bin:latest

# Verify binary name matches your project's output
COPY --from=builder /healer-network/target/release/healer-network-node /usr/local/bin

USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /healer-network healer-network && \
	mkdir -p /data /healer-network/.local/share/healer-network-node && \
	chown -R healer-network:healer-network /data /healer-network/.local && \
	ln -s /data /healer-network/.local/share/healer-network-node/chains && \
	# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
	# check if executable works in this container
	/usr/local/bin/healer-network-node --version

USER healer-network

# Default ports for Substrate nodes
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

# Set default arguments to specify the correct data path and network binding
ENTRYPOINT ["/usr/local/bin/healer-network-node"]
CMD ["--base-path=/data", "--chain=dev", "--rpc-cors=all", "--unsafe-rpc-external", "--rpc-methods=unsafe", "--rpc-external", "--validator", "--alice"]
