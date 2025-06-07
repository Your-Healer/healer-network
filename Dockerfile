FROM docker.io/paritytech/ci-unified:latest as builder

WORKDIR /polkadot
COPY . /polkadot

RUN apt-get update && apt-get install -y

RUN cargo fetch
RUN cargo build --locked --release

FROM docker.io/parity/base-bin:latest

# Remove old binary if it exists
RUN rm -f /usr/local/bin/healer-network-node

COPY --from=builder /polkadot/target/release/healer-network-node /usr/local/bin

USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /polkadot polkadot && \
	mkdir -p /data /polkadot/.local/share/healer-network-node && \
	chown -R polkadot:polkadot /data /polkadot/.local && \
	ln -s /data /polkadot/.local/share/healer-network-node/chains && \
	# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
	# Clean up package cache
	apt-get clean && \
	rm -rf /var/lib/apt/lists/* && \
	# check if executable works in this container
	/usr/local/bin/healer-network-node --version

USER polkadot

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/healer-network-node"]