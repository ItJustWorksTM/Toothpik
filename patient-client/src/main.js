import '@babel/polyfill';
import 'mutationobserver-shim';

import { BootstrapVue, BootstrapVueIcons } from 'bootstrap-vue';
import Vue from 'vue';

import './assets/style.scss';
import 'leaflet/dist/leaflet.css';

import App from './App.vue';
import router from './router';
import store from './store';

Vue.use(BootstrapVue);
Vue.use(BootstrapVueIcons);

Vue.config.productionTip = false;
Vue.config.errorHandler = (msg, vm, info) => {
  // will create nice component for this later
  // console.log(`${msg}! \r\n Please Try Again`);
};

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app');
