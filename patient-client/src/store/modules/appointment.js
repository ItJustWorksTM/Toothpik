import cbor from 'cbor';
import mqtt from '../../mqtt/api';
import router from '../../router';

export default {
  state: {
    bookedSlot: null,
    time: '',
    dentistid: null,
    clinicName: ''
  },
  mutations: {
    book_time: s => {
      router.push('/bookedtime');
    },
    book: (state, id) => {
      state.dentistid = id;
      router.push('/timeslot');
    },
    appointment: () => {
      router.push('/appointment');
    },
    booked_slot: (state, payload) => {
      state.bookedSlot = payload;
    }
  },
  actions: {
    gbook: async ({ dispatch, rootState }, { variant, payload }) => {
      if(!mqtt.authenticated)
        return dispatch('toast', { message: 'Oops! You\'re not logged in, log in and try again.', type: 'error', variant: 'danger' }, { root: true });

      rootState.loading++;

      const res = await mqtt.request(`store/appointment/${mqtt.client_id}/${variant}`,
        `client/${mqtt.client_id}/reply/store/appointment/${variant}`,
        cbor.encode(payload));

      rootState.loading--;

      if(!res || res.time === 'none')
        return dispatch('toast', { message: 'Oops! Unable to make a booking at this moment, reload the page and try again.', type: 'error', variant: 'danger' }, { root: true });

      if(res.error)
        return dispatch('toast', { message: res.error, type: 'error', variant: 'danger' }, { root: true });

      // TODO: set state
      router.push('bookedtime');
    },

    custom_book: ({ state, dispatch }, data) => {
      if(!state.dentistid)
        return dispatch('toast', { message: 'No active dentist selected', type: 'error', variant: 'danger' }, { root: true });
      const payload = {
        userid: mqtt.client_id,
        dentistid: state.dentistid,
        issuance: 0,
        time: `${state.bookedSlot.date} ${state.bookedSlot.time}`,
        ...data
      };
      console.log('custom_book', payload);
      dispatch('gbook', { variant: 'book', payload });
    },

    quick_book: ({ state, dispatch }, { date, time }) => {
      if(!state.dentistid)
        return dispatch('toast', { message: 'No active dentist selected', type: 'error', variant: 'danger' }, { root: true });
      time = time[0] === ' ' ? time : ` ${time}`;
      const payload = {
        userid: mqtt.client_id,
        requestid: '1',
        dentistid: state.dentistid,
        issuance: 0,
        time: `${date}${time}`
      };
      dispatch('gbook', { variant: 'quick_book', payload });
    }
  }
};
