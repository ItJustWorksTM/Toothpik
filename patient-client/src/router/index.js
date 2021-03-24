import Vue from 'vue';
import VueRouter from 'vue-router';

import Appointment from '../views/Appointment.vue';
import Auth from '../views/Auth.vue';
import BookedTime from '../views/bookedslot.vue';
import Home from '../views/Home.vue';
import TimeSlot from '../views/TimeSlot.vue';
import Verification from '../views/Verification.vue';

import mqtt from '../mqtt/api';

Vue.use(VueRouter);

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/appointment',
    name: 'Appointment',
    component: Appointment,
    beforeEnter(to, from, next) {
      if(mqtt.authenticated)
        next();
      else
        next('/auth');
    }
  },
  {
    path: '/auth',
    name: 'Auth',
    component: Auth
  },
  {
    path: '/timeslot',
    name: 'TimeSlot',
    component: TimeSlot
  },
  {
    path: '/bookedtime',
    name: 'BookedTime',
    component: BookedTime,
    beforeEnter(to, from, next) {
      if(mqtt.authenticated)
        next();
      else
        next('/auth');
    }
  },
  {
    path: '/verification',
    name: 'Verification',
    component: Verification
  }
];

const router = new VueRouter({
  routes
});

export default router;
