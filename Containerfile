ARG FEDORA_VERSION=45

FROM quay.io/fedora/fedora:${FEDORA_VERSION} AS build
RUN dnf -y install rust cargo && dnf clean all
WORKDIR /src
COPY . .
RUN cargo build --release --locked

FROM ghcr.io/sorabbyngo-os/fedora-base:f${FEDORA_VERSION}
COPY --from=build /src/target/release/aiond /usr/bin/aiond
COPY packaging/aion.service /usr/lib/systemd/system/aion.service
RUN systemctl preset aion.service
RUN bootc container lint --fatal-warnings
