<template>
  <div>
    <p class="my-4 text-center">
      Don't you have an account ?<b-link
        class="text-success font-weight-bold"
        @click="signupForm"
      >
        Create Account
      </b-link>. It takes lesser than
      a minute
    </p>
    <b-form
      autocomplete="off"
      @submit="onSubmit"
    >
      <div class="form-input-group">
        <b-form-input
          id="username"
          v-model="formData.username"
          type="text"
          required
          class="form-input"
        />
        <label
          for="username"
          class="form-label"
        ><span class="label-content">Username</span></label>
      </div>
      <div class="form-input-group">
        <b-form-input
          id="password"
          v-model="formData.password"
          required
          type="password"
          class="form-input"
        />
        <label
          for="password"
          class="form-label"
        ><span class="label-content">Password</span>
        </label>
      </div>
      <div class="form-input-group">
        <b-form-input
          id="totp_code"
          v-model="formData.totp_code"
          required
          type="number"
          class="form-input"
        />
        <label
          for="totp_code"
          class="form-label"
        ><span class="label-content">2FA code (TOTP)</span>
        </label>
      </div>
      <b-form-checkbox
        id="remember"
        v-model="formData.status"
        name="checkbox"
        value="true"
        unchecked-value="false"
        class="mt-3"
      >
        Remember me
      </b-form-checkbox>

      <b-button
        class="mt-4 mb-5"
        pill
        type="submit"
        variant="success"
        block
        :disabled="this.$store.state.loading > 0"
      >
        Login
      </b-button>
    </b-form>
  </div>
</template>

<script>
export default {
  data() {
    return {
      formData: {
        username: '',
        password: '',
        totp_code: '',
        status: ''
      }
    };
  },
  methods: {
    onSubmit(evt) {
      evt.preventDefault();
      this.$store.dispatch('login', this.formData);
    },
    signupForm() {
      this.$store.commit('login_form');
    }
  }
};
</script>
