<template>
  <div class="auth-container shadow">
    <div
      class="d-flex content-container"
    >
      <div
        class="d-none d-lg-inline image-container"
      >
        <img
          src="../assets/brush.jpg"
          alt="doctor"
          class="dentist-img"
        >
      </div>
      <div class="mt-5 form-container">
        <div class=" auth-forms-container">
          <div class="d-flex auth-forms">
            <b-link
              class="text-primary form-links text-decoration-none pb-2 px-4"
              :active="LogInForm"
              @click="loginForm"
            >
              Login
            </b-link>
            <b-link
              class="text-success form-links text-decoration-none pb-2 px-4"
              :active="!LogInForm"
              @click="signupForm"
            >
              Sign up
            </b-link>
          </div>
          <div v-if="LogInForm">
            <Login />
          </div>
          <div v-else>
            <Signup />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import Login from '../components/auth/Login.vue';
import { mapGetters } from 'vuex';
import Signup from '../components/auth/Signup.vue';

export default {
  components: {
    Login,
    Signup
  },
  data() {
    return {
    };
  },
  computed: {
    ...mapGetters([ 'LogInForm' ])
  },
  methods: {
    loginForm() {
      if(!this.LogInForm)
        this.$store.commit('login_form');
    },
    signupForm() {
      if(this.LogInForm)
        this.$store.commit('login_form');
    }
  }
};
</script>

<style scoped>
@import '../assets/form.css';
.auth-forms {
  justify-content: space-around;
  font-weight: 700;
  font-size: 1.25rem;
}
.auth-container {
  margin: auto;
  width: 90%;
  height: 100%;
}
.auth-forms-container{
  width: 70% !important;
  margin-left: auto;
  margin-right: auto;
}
.content-container{
  justify-content: space-between;
}
.dentist-img{
  height: 100%;
  width: 100%;
  object-fit: cover;
}
.image-container{
  width: 40%;
}
.form-container{
  width: 60%;
}
.form-links.active{
  border-bottom: 3px solid;
}
@media screen and (max-width: 992px){
  .form-container{
  width: 100%;
}
}
</style>
