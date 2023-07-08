# bluetooth-mqtt-notifier

This tools scans for bluetooth devices and sends notifications to a mqtt topic if it found the given UUIDs.
I use this to detect if the bluetooth tokens on our keyrings are in our house and to use this information in our smarthome system, e.g. to turn lights off if nobody is at home

## Usage

```
export MQTT_HOST="tcp://1.2.34:1883"
export MQTT_USER="some-user"
export MQTT_PASSWORD="some-assword"
export MQTT_TOPIC="presence/bluetooth"
export SEARCH_UUIDS="45678123,90876543"
./bluetooth-mqtt-notifier
```

## Build

Build using `rust` and `cargo` (you will need some dev libraries installed in your system):

```
cargo build -r
```

Or you use the docker container I prepared for building this:


```
docker build -f Dockerfile.build .
# you can use the build result and use it in another container to run the executable
docker build -t bluetooth-mqtt-notifier:latest .
```

