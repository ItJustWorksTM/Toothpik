import cbor from 'cbor';
import mqtt from '../../mqtt/api';
import router from '@/router';

export default {
  state: {
    userId: '',
    username: null,
    isLoggedForm: true,
    isAuth: mqtt.authenticated
  },
  getters: {
    LogInForm: s => s.isLoggedForm,
    isAuthorized: s => s.isAuth
  },
  mutations: {
    login_form: s => {
      s.isLoggedForm = !s.isLoggedForm;
    },
    sign_up: s => {
      s.isLoggedForm = !s.isLoggedForm;
    }
  },
  actions: {
    login: async ({ state, commit, rootState, dispatch }, payload) => {
      rootState.loading++;
      const res = await mqtt.login(payload.username, `${payload.password}\x0A${payload.totp_code}`);
      rootState.loading--;

      if(!res)
        return dispatch('toast', { message: 'Login Failed', type: 'error', variant: 'danger' }, { root: true });

      router.push('/');
      state.isAuth = true;
    },

    signup: async ({ rootState, state, dispatch, commit }, payload) => {
      rootState.loading++;
      const res = await mqtt.request(`store/user/public/${mqtt.client_id}/register`,
        `client/${mqtt.client_id}/reply/store/user/public/register`, cbor.encode(payload));
      rootState.loading--;

      if(!res || res.error)
        return dispatch('toast', { message: 'Registration failed', type: 'error', variant: 'danger' }, { root: true });

      state.username = payload.username;

      if(rootState.us_features.includes('mail_check'))
        router.push('/verification');

      commit('login_form');
      dispatch('toast', { message: 'Registration Succeeded', type: 'Succeed', variant: 'success' }, { root: true });
    },

    verify: async ({ dispatch, commit }, payload) => {
      const res = await mqtt.request(`store/user/public/${mqtt.client_id}/validate`,
        `client/${mqtt.client_id}/reply/store/user/public/validate`, cbor.encode(payload));

      if(!res || res.error)
        return dispatch('toast', { message: 'Verification failed', type: 'error', variant: 'danger' }, { root: true });

      commit('login_form');
      router.push('/auth');

      dispatch('toast', { message: 'Registration Succeeded', type: 'Succeed', variant: 'success' }, { root: true });
    },

    logout: async ({ state }) => {
      await mqtt.disconnect();
      state.username = null;
      state.isAuth = false;
    }
  }
};
