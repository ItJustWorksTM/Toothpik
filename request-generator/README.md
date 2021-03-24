# Toothpik Hammer
_Booking requests generator_

### Requirements

The hammer needs Python 3.8 or newer, as well as the libraries listed in the following `pip3` invocation:
```
pip3 install paho.mqtt uuid cbor2 argparse matplotlib
```

### Usage

```
$ python3 main.py --help
usage: main.py [-h] [--hammer-count HAMMER_COUNT] [-si STATS_INTERVAL] [--headless] broker_addr

positional arguments:
  broker_addr           Address of the MQTT broker that the Toothpik system is connected to

optional arguments:
  -h, --help            show this help message and exit
  --hammer-count HAMMER_COUNT
                        Number of users
  -si STATS_INTERVAL, --stats-interval STATS_INTERVAL
                        interval between stats sampling (seconds)
  --headless
```
