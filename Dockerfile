FROM docker.io/paritytech/ci-unified:latest as builder

WORKDIR /healer-network-polkadot
COPY . /healer-network-polkadot

RUN apt-get update && apt-get install -y

RUN cargo fetch
RUN cargo build --locked --release

FROM docker.io/parity/base-bin:latest

COPY --from=builder /healer-network-polkadot/target/release/healer-network-node /usr/local/bin

USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /healer-network-polkadot healer-network-polkadot && \
	mkdir -p /data /healer-network-polkadot/.local/share/healer-network-node && \
	chown -R healer-network-polkadot:healer-network-polkadot /data /healer-network-polkadot/.local && \
	ln -s /data /healer-network-polkadot/.local/share/healer-network-node/chains && \
	# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
	# check if executable works in this container
	/usr/local/bin/healer-network-node --version

USER healer-network-polkadot

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/healer-network-node"]