<template>
  <div class="auth-container shadow d-flex justify-content-between">
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
        <h2 class="text-success text-center">
          Verify Yourself
        </h2>
        <p class="mt-4 text-center mb-5">
          To proceed please enter the verification code you received <br> in your registered email.
        </p>
        <b-form
          autocomplete="off"
          @submit="onSubmit"
        >
          <div class="form-input-group">
            <b-form-input
              id="code"
              v-model="verificationCode"
              type="text"
              required
              class="form-input"
            />
            <label
              for="code"
              class="form-label"
            ><span class="label-content">Verification Code</span></label>
            <b-form-text class="mt-0 pt-0">
              Have not received the code? Please check your spam folder
            </b-form-text>
          </div>
          <b-button
            class="my-5"
            pill
            type="submit"
            variant="success"
            block
          >
            Verify
          </b-button>
        </b-form>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      verificationCode: ''
    };
  },
  methods: {
    onSubmit(evt) {
      evt.preventDefault();
      this.$store.dispatch('verify', { username: this.$store.state.auth.username, verification_code: parseInt(this.verificationCode, 16) });
    }
  }
};
</script>

<style scoped>
@import '../assets/form.css';
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
@media screen and (max-width: 992px){
  .form-container{
  width: 100%;
}
}
</style>
