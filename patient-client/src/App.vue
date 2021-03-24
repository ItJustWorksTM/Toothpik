<template>
  <div id="app">
    <LoaderScreen />
    <NavBar />
    <router-view />
    <ContentFooter />
  </div>
</template>

<script>
import ContentFooter from './components/ContentFooter.vue';
import LoaderScreen from './components/LoaderScreen.vue';
import NavBar from './components/NavBar.vue';

export default {
  components: {
    NavBar,
    ContentFooter,
    LoaderScreen
  },
  created() {
    this.$store.dispatch('connect_mqtt');
    this.$store.watch(state => state.toastMessages,
      toastMessages => {
        this.$bvToast.toast(toastMessages.message, {
          title: `${toastMessages.type.toUpperCase()} !`,
          autoHideDelay: 3000,
          toaster: 'b-toaster-top-center',
          variant: toastMessages.variant
        });
      });
  }
};
</script>

<style>
#app {
  font-family:'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}

#nav {
  padding: 30px;
}

#nav a {
  font-weight: bold;
  color: #2c3e50;
}

#nav a.router-link-exact-active {
  color: #42b983;
}
</style>
