import cbor from 'cbor';
import { DateTime } from 'luxon';
import mqtt from '../../mqtt/api';

export default {
  state: {
    // Hack to save the current week
    current_week_availability: {},
    availability: {},
    current_week: 0
  },
  getters: {
    is_week_available: state => dentistid => {
      const dentist = state.current_week_availability[dentistid];

      if(!dentist)
        return false;

      return dentist.filter(e => e.time.filter(d => d.available).length > 0).length > 0;
    }
  },
  actions: {
    init_availability: async ({ state, dispatch, commit }) => {
      dispatch('availability');
      await mqtt.listen('store/appointment/public/realtime/availability', avail => {
        console.log('New availability update received: ', avail);

        const updateCamel = target => {
          const dentist = target[avail.dentistid];

          // If we don't have the dentist we can safely ignore it as we aren't showing it to the user anyways
          if(!dentist)
            return;

          // Find indices of the date and time, then
          const date = dentist.findIndex(i => i.date === avail.date);

          if(date === -1)
            return;

          // Update the specfic time
          const time = dentist[date].time.findIndex(i => i.time === avail.time);

          target[avail.dentistid][date].time[time].available = avail.available;
          return target;
        };
        // trigger reactivity update
        state.availability = { ...updateCamel(state.availability) };

        // Hack to save the current week
        state.current_week_availability = { ...updateCamel(state.current_week_availability) };
      });
    },

    next_week: ({ state, dispatch }) => {
      dispatch('set_week', ++state.current_week);
    },
    prev_week: ({ state, dispatch }) => {
      dispatch('set_week', --state.current_week);
    },
    set_week: ({ state, dispatch }, weekno) => {
      state.current_week = weekno;
      if(state.current_week < 0)
        state.current_week = 0;
      dispatch('availability');
    },
    // Frontend components send amount of weeks they want to see in to the future
    // compared to the current date, so weekno 0 would give the current day + 7 days
    // This results in 5 available always.
    availability: async ({ state, dispatch, rootState }) => {
      rootState.loading++;
      for(const dentist of rootState.dentists) {
        const now = DateTime.local();
        const startDate = now.plus({ weeks: state.current_week }).toFormat('y-LL-dd');
        const endDate = now.plus({ weeks: state.current_week + 1 }).minus({ days: 1 }).toFormat('y-LL-dd');

        const avail = await mqtt.request(`store/appointment/public/${mqtt.client_id}/availability`,
          `client/${mqtt.client_id}/reply/store/appointment/public/availability`,
          cbor.encode({
            dentistid: dentist.id,
            start_date: startDate,
            end_date: endDate
          }));

        if(!avail) {
          dispatch('toast', { message: 'Failed to fetch availability', type: 'error', variant: 'danger' }, { root: true });
          continue;
        }

        const filtered = avail.availability.filter(e => e.time.length !== 0);

        // Hack to save the current week
        if(state.current_week === 0) {
          state.current_week_availability[avail.dentistid] = filtered;
          state.current_week_availability = { ...state.current_week_availability };
        }

        state.availability[avail.dentistid] = filtered;
        state.availability = { ...state.availability };
      }
      rootState.loading--;
      console.log(state.availability);
    }
  }
};
