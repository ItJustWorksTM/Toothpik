import Appointment from './modules/appointment';
import Auth from './modules/auth.js';
import Availability from './modules/availability';
import cbor from 'cbor';
import mqtt from '../mqtt/api';
import Vue from 'vue';
import Vuex from 'vuex';

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    loading: 0,
    client: null,
    dentistId: '',
    dentists: [],
    us_features: [],
    toastMessages: {}
  },
  mutations: {
    add_toast_data: (state, toastMessage) => (state.toastMessages = toastMessage)
  },
  actions: {
    connect_mqtt: async ({ state, dispatch }) => {
      state.loading++;
      const res = await mqtt.connect_anon(); // Just to activate it :)
      state.loading--;

      if(!res)
        return dispatch('toast', { message: 'Failed to connect', type: 'error', variant: 'danger' });
      await dispatch('features');
      dispatch('dentistData');

      // Realtime subs
      await mqtt.listen('store/dentist/public/realtime/registry', dt => {
        console.log('New dentist registry received: ', dt.dentists);
        state.dentists = dt.dentists;

        // When the dentist registry changes we need to request the whole
        // availability structure as we dont get full updates for that.
        dispatch('availability');
      });
    },

    dentistData: async ({ state, dispatch }, payload) => {
      state.loading++;
      const res = await mqtt.request(`store/dentist/public/${mqtt.client_id}/registry`, `client/${mqtt.client_id}/reply/store/dentist/public/registry`, cbor.encode(payload));
      if(!res)
        return dispatch('toast', { message: 'Failed to fetch dentist info', type: 'error', variant: 'danger' });
      state.dentists = res.dentists;
      // Availability depends on the dentist data
      // So init once we have that
      dispatch('init_availability');

      state.loading--;
    },
    toast: ({ commit }, payload) => {
      commit('add_toast_data', payload);
    },
    features: async ({ state }) => {
      console.log('Available features before: ', state.us_features);
      state.us_features =
      await mqtt.request(`store/user/public/${mqtt.client_id}/features`,
        `client/${mqtt.client_id}/reply/store/user/public/features`, '') ||
        [ 'reg_captcha', 'mail_check' ]; // If unreachable, assume all features
      console.log('Available features: ', state.us_features);
    }
  },
  modules: {
    auth: Auth,
    appointment: Appointment,
    availability: Availability
  }
});
