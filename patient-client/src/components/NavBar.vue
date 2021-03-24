<template>
  <div>
    <b-navbar
      toggleable="md"
      class="navbar-container"
    >
      <b-navbar-brand>
        <b-link
          to="/"
          class="app-brand"
        >
          TOOTH PIK
          <div class="app-brand-undertext">
            DENTAL SERVICES
          </div>
        </b-link>
      </b-navbar-brand>
      <b-navbar-toggle target="sidebar-right" />
      <b-collapse
        is-nav
        class="d-none"
      >
        <UserControlBtn />
      </b-collapse>
      <b-sidebar
        id="sidebar-right"
        class="d-md-none"
        right
        shadow
        backdrop
      >
        <UserControlBtn />
      </b-sidebar>
    </b-navbar>
  </div>
</template>

<script>
import UserControlBtn from './UserControlBtns.vue';
export default {
  name: 'NavBar',
  components: {
    UserControlBtn
  }
};
</script>

<style scoped>
.app-brand {
  font-weight: 700;
  font-size: 2rem;
  color: var(--black);
}
.navbar-container{
  width: 92%;
  margin: auto;
  margin-bottom: 1.5rem;
  margin-top: 1.5rem;
}
.app-brand:hover {
  text-decoration: none;
}
.app-brand-undertext{
  color: var(--indigo);
  font-size: 1.25rem;
  line-height: 0.5rem;
}
</style>
