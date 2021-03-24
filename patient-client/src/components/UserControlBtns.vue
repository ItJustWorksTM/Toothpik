<template>
  <b-navbar-nav class="ml-auto my-5 d-flex flex-column text-center flex-md-row my-md-0">
    <b-nav-item
      v-if="isAuthorized"
      to="/appointment"
      class="mb-5 mb-md-0 mr-md-5"
    >
      <span class="app-nav-item">APPOINTMENT</span>
    </b-nav-item>
    <b-nav-item
      v-if="isAuthorized"
      class="mb-5 mb-md-0 mr-md-5"
      @click="logOut"
    >
      <span class="app-nav-item">SIGN OUT</span>
    </b-nav-item>
    <b-nav-item
      v-else
      to="/auth"
      class="mb-5 mb-md-0 mr-md-5"
    >
      <span class="app-nav-item">SIGN IN</span>
    </b-nav-item>
  </b-navbar-nav>
</template>

<script>
import { mapGetters } from 'vuex';

export default {
  computed: {
    ...mapGetters([ 'isAuthorized' ])
  },
  methods: {
    logOut() {
      this.$store.dispatch('logout');
    }
  }
};
</script>

<style scoped>
.app-nav-item {
  color: var(--indigo);
  font-weight: 700;
  font-size: 1.25rem;
}
</style>
