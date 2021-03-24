import string
import time
import json
import random
import paho.mqtt.client as mqtt
from uuid import uuid4
from threading import Thread, Timer, Event, current_thread
import cbor2 as cbor
import matplotlib.pyplot as plt
import argparse

parser = argparse.ArgumentParser()
parser.add_argument("broker_addr", help="Address of the MQTT broker that the Toothpik system is connected to")
parser.add_argument("--hammer-count", default=10, type=int, help="Number of users")
parser.add_argument("-si", "--stats-interval", default=1, type=int, help="interval between stats sampling (seconds)")
parser.add_argument("--headless", action="store_true")
args = parser.parse_args()


stats_sample_time = args.stats_interval;

random.seed(int(time.time()))

def my_log(s):
    # print(s)
    pass


def usuff_gen(size=6, chars=string.ascii_uppercase + string.digits):
    return "".join(random.choice(chars) for _ in range(size))


topic_register_req = "store/user/public/%s/register"
topic_register_res = "client/%s/reply/store/user/public/register"

topic_myid_req = "store/user/%s/my_id"
topic_myid_res = "client/%s/reply/store/user/my_id"

topic_quickbook_res = "client/%s/reply/store/appointment/quick_book"
topic_quickbook_req = "store/appointment/%s/quick_book"


class ReapeatingTimer(Thread):
    def __init__(self, interval, stop_event, callback):
        Thread.__init__(self)
        self.interval = interval
        self.stopped = stop_event
        self.callback = callback

    def run(self):
        while not self.stopped.wait(self.interval):
            self.callback()


def on_connect(client, userdata, flags, rc):
    # print("Connected with result code "+str(rc))
    if userdata.anonymous:
        my_log("Connected as anonymous")
        result, _ = client.subscribe(topic_register_res % userdata.conn_id)
        assert result == mqtt.MQTT_ERR_SUCCESS
        client.publish(topic_register_req % userdata.conn_id, cbor.dumps(
            {"id": "", "name": "Hammer", "username": userdata.username, "email": "%s@domain.tld" % userdata.username, "secret": userdata.password}))
        my_log("Sent registeration")
    else:
        my_log("Connected as elevated")
        client.subscribe(topic_myid_res % userdata.conn_id)
        client.publish(topic_myid_req % userdata.conn_id, cbor.dumps({"username": userdata.username}))
        my_log("Sent my_id")


def on_disconnect(client, userdata, rc):
    if rc != 0:
        print("Unexpected disconnection.")
    pass


def on_log(client, userdata, level, buf):
    # print("LOG [" + str(level) + "]: " + buf)
    pass


sec_out = 0
sec_in = 0

def on_message(client, userdata, msg):
    # print("Message: "+msg.topic+" "+str(cbor.loads(msg.payload)))
    if userdata.anonymous and msg.topic == topic_register_res % userdata.conn_id:
        client.unsubscribe(topic_register_res % userdata.conn_id)
        client.disconnect()
        client.loop_stop()
        payload = cbor.loads(msg.payload)
        if "error" in payload:
            print("Server error: " + payload["error"])
            return
        my_log("Registered")
        userdata.sys_id = payload["id"]
        client = mqtt.Client(userdata.conn_id)
        userdata.client = client
        client.user_data_set(userdata)
        client.on_log = on_log
        client.on_connect = on_connect
        client.on_message = on_message
        client.on_disconnect = on_disconnect
        client.username_pw_set(userdata.username, userdata.password)
        userdata.anonymous = False
        client.connect("aerostun.dev")
        client.loop_start()
    elif (not userdata.anonymous) and (msg.topic == topic_myid_res % userdata.conn_id):
        my_log("Got my id")
        assert userdata.sys_id == cbor.loads(msg.payload)["id"]
        client.subscribe(topic_quickbook_res % userdata.sys_id)
        userdata.done.set()
    elif (not userdata.anonymous) and (msg.topic == topic_quickbook_res % userdata.sys_id):
        global sec_in
        sec_in += 1
        userdata.total_recv += 1


def quick_book(shard):
    global sec_out
    #res = shard.client.publish(topic_quickbook_req % shard.sys_id, json.dumps({
    #   "userid": "12345678-1234-1234-1234-123456789abc",
    #   "requestid": str(uuid4()),
    #   "dentistid": random.randint(0, 10),
    #   "issuance": int(time.time()),
    #   "time": time.strftime("%F %H:%M")
    #}))
    res = shard.client.publish(topic_quickbook_req % shard.sys_id, cbor.dumps({
       "userid": "12345678-1234-1234-1234-123456789abc",
       "requestid": str(uuid4()),
       "dentistid": random.randint(0, 5),
       "date": time.strftime("%F %H:%M")
    }))
    if res.rc != mqtt.MQTT_ERR_SUCCESS:
        print(res)
        exit(1)
    sec_out += 1
    shard.total_sent += 1
    pass


class HammerShard:
    @staticmethod
    def spawn():
        shard = HammerShard()
        th = Thread(target=shard.start)
        th.start()
        return shard

    def __init__(self):
        self.done = Event()
        self.ping = Event()
        self.command = ""
        self.command_arg = 0

    def start(self):
        self.thread = current_thread()
        self.sys_id = ""
        self.anonymous = True
        self.total_sent = 0
        self.total_recv = 0

        self.conn_id = str(uuid4())
        my_log("I am " + self.conn_id)

        self.username = "hammer_" + usuff_gen()
        self.password = "quxx"

        self.client = mqtt.Client(self.conn_id)
        self.client.user_data_set(self)
        self.client.on_log = on_log
        self.client.on_connect = on_connect
        self.client.on_message = on_message
        self.client.on_disconnect = on_disconnect

        self.client.connect(args.broker_addr)
        self.client.loop_start()
        self.done.wait()
        while True:
            self.ping.wait()
            self.ping.clear()
            if self.command == "stop":
                self.client.loop_stop()
                break
            elif self.command == "quick_book":
                for _ in range(0, self.command_arg):
                    quick_book(self)
        pass


class Hammer:
    def __init__(self, nshards):
        self.shards = []
        for _ in range(nshards):
            self.shards.append(HammerShard.spawn())
        pass

    def wait_for_readiness(self):
        for shard in self.shards:
            shard.done.wait()
        pass

    def stop(self):
        for shard in self.shards:
            shard.command = "stop"
            shard.ping.set()
        for shard in self.shards:
            shard.thread.join()
        pass

    def quickn(self, count):
        for shard in self.shards:
            shard.command = "quick_book"
            shard.command_arg = count
            shard.ping.set()
        pass

    def load(self, interval_seconds, runs, nails, last=True):
        for _ in range(0, runs):
            self.quickn(nails)
            time.sleep(interval_seconds)
        if last:
          time.sleep(interval_seconds * nails)
        pass


ser_out = []
ser_in = []
def wrangle_stats():
  global sec_out
  global sec_in
  ser_out.append(sec_out)
  sec_out = 0
  ser_in.append(sec_in)
  sec_in = 0

if __name__ == '__main__':
    ctx = Hammer(args.hammer_count)
    ctx.wait_for_readiness()
    print("All shards ready")

    stop_timer = Event()
    stats_tim = ReapeatingTimer(stats_sample_time, stop_timer, wrangle_stats)
    stats_tim.start()

    ctx.load(interval_seconds=1, runs=15 * 60, nails=10)

    stop_timer.set()
    ctx.stop()

    total_requests = 0
    for shard in ctx.shards:
        total_requests += shard.total_sent
    total_responses = 0
    for shard in ctx.shards:
        total_responses += shard.total_recv

    print("Results: Sent %d | Received %d | Failure rate %d%%" % (total_requests, total_responses, 100*(total_requests-total_responses)/total_requests))

    print(ser_in, ser_out)
    out_gl, = plt.plot(ser_out, color='r')
    out_gl.set_label('Outbound')
    in_gl, = plt.plot(ser_in, color='g')
    in_gl.set_label('Inbound')
    plt.xlabel('Time (s)')
    plt.ylabel('Artificial booking traffic (req/s)')
    plt.figlegend()

    if args.headless:
      plt.savefig('booking_traffic.png')
    else:
      plt.show()
