/* eslint-disable no-console */
import cbor from 'cbor';
import mqtt from 'async-mqtt';
import { v4 } from 'uuid';

export default {
  client: null,
  client_id: null,
  authenticated: false,
  url: 'wss://aerostun.dev:8884',
  rl_listeners: [],
  topics: [],
  free_responses: [],
  // TODO: introduce lock to block multiple connection tries.

  async disconnect() {
    if(this.client)
      await this.client.end();
    this.client = null;
    this.authenticated = null;
    this.topics = [];
  },

  async connect_anon() {
    console.log(`connecting on ${this.url}`);

    await this.disconnect();

    const id = v4();
    const client = await mqtt
      .connectAsync(this.url,
        {
          clientId: id,
          keepalive: 60,
          reconnectPeriod: 0,
          qos: 1
        },
        false)
      .catch(() => null);

    if(!client) {
      console.log('Failed to connect!');
      return false;
    }

    console.log('Connection acquired, subbing');
    this.client = client;
    this.client_id = id;
    this.client.on('message', (topic, message, stuff) =>
      this.realtime_handler(topic, message, stuff));
    await this.sub_topic(`client/${this.client_id}/reply/#`);

    await this.resubscribe();
    console.log('Connected as anon');

    return true;
  },

  async get_reply(ontopic, timeout) {
    if(!this.client)
      return null;

    let timedout = false;
    setTimeout(() => {
      timedout = true;
    }, timeout);

    while(true) {
      await new Promise(resolve =>
        this.client.once('message', () => resolve()));

      const response = this.free_responses.findIndex(e => e.topic === ontopic);
      if(response === -1) {
        if(timedout)
          return null;
        continue;
      }

      const ret = this.free_responses[response].message;
      this.free_responses.splice(response, 1);

      return ret;
    }
  },

  async request(onTopic, replyTopic, payload) {
    if(!this.client || !onTopic || !replyTopic)
      return false;

    console.log(`Requesting on topic: ${onTopic}`);

    // TODO: Unsure if publishing and then quickly listening actually works
    const pub = await this.client.publish(onTopic, payload).catch(() => null);

    if(pub === null) {
      console.log('Failed to publish');
      return false;
    }

    const message = await this.get_reply(replyTopic, 5000);
    if(!message)
      return false;
    console.log(`Reply on: ${replyTopic} with body: \n`, cbor.decode(message));
    return cbor.decode(message);
  },

  async connect_auth(username, password) {
    if(!this.client)
      return false;
    await this.client.end();
    const client = await mqtt
      .connectAsync(this.url,
        {
          clientId: this.client_id,
          username,
          password,
          keepalive: 60,
          reconnectPeriod: 0,
          qos: 1
        },
        false)
      .catch(() => null);

    if(!client)
      return false;

    this.client = client;
    this.authenticated = true;

    this.client.on('message', (topic, message) =>
      this.realtime_handler(topic, message));

    console.log('Authenticated with: ', username);
    return true;
  },

  async login(username, password) {
    console.log('Trying to login with: ', username);
    if(!(await this.connect_auth(username, password))) {
      console.log('Authentication failed');
      return false;
    }

    if(
      !(await this.sub_topic(`client/${this.client_id}/reply/store/user/my_id`))
    )
      return false;

    const uuid = await this.request(`store/user/${this.client_id}/my_id`,
      `client/${this.client_id}/reply/store/user/my_id`,
      cbor.encode({ username }));

    if(!uuid)
      return false;

    this.client_id = uuid.id;

    await this.sub_topic(`client/${this.client_id}/reply/#`);

    await this.resubscribe();

    console.log('Logged in with: ', username);
    return true;
  },

  realtime_handler(topic, message, stuff) {
    const decoded = cbor.decode(message);

    if(!decoded)
      return;

    console.log(`Realtime: Message on: ${topic}: `, decoded, stuff);

    const handlers = this.rl_listeners.filter(e => e.topic === topic);
    handlers.forEach(e => e.callback(decoded));

    if(handlers.length === 0)
      this.free_responses.push({ topic, message, stuff });
  },

  async resubscribe() {
    const subbed = [];
    for(const topic of this.topics) {
      const result = await this.client
        .subscribe(topic, { qos: 1 })
        .catch(() => null);
      if(result !== null) {
        if(result.length > 0)
          subbed.push(topic);
      }
    }
    this.topics = subbed;
  },

  async sub_topic(topic) {
    const result = await this.client
      .subscribe(topic, { qos: 1 })
      .catch(() => null);
    if(result === null)
      return false;

    if(result.length === 0)
      return false;

    this.topics.push(topic);

    return true;
  },

  async listen(topic, callback) {
    if(!(await this.sub_topic(topic)))
      return false;
    console.log(`Listener for ${topic} attached`);
    this.rl_listeners.push({ topic, callback });
    return true;
  }
};
