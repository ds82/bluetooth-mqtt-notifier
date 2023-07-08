FROM debian:stable-slim

RUN apt update && apt install --yes libdbus-1-dev libssl-dev

COPY target/release/bluetooth-mqtt-notifier /bluetooth-mqtt-notifier

ENTRYPOINT /bluetooth-mqtt-notifier
