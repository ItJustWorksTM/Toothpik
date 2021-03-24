<template>
  <div>
    <p class="my-4 text-center">
      Enter the following details to create an account
    </p>
    <b-form
      autocomplete="off"
      @submit="onSubmit"
    >
      <div class="form-input-group">
        <b-form-input
          id="name"
          v-model="formData.name"
          type="text"
          required
          class="form-input"
        />
        <label
          for="name"
          class="form-label"
        ><span class="label-content">Name</span></label>
      </div>
      <div class="form-input-group">
        <b-form-input
          id="email"
          v-model="formData.email"
          type="email"
          required
          class="form-input"
        />
        <label
          for="email"
          class="form-label"
        ><span class="label-content">Email</span></label>
      </div>
      <div class="form-input-group">
        <b-form-input
          id="username"
          v-model="formData.username"
          type="text"
          required
          class="form-input"
          @blur="set2faUrl"
        />
        <label
          for="username"
          class="form-label"
        ><span class="label-content">Username</span></label>
      </div>
      <div class="form-input-group">
        <b-form-input
          id="password"
          v-model="formData.secret"
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
      <vue-hcaptcha
        v-if="captchaEnabled"
        sitekey="10000000-ffff-ffff-ffff-000000000001"
        @verify="onVerify"
        @expired="onExpired"
      />
      <b-button
        class="my-5"
        pill
        type="submit"
        variant="success"
        block
        :disabled="this.$store.state.loading > 0
          || (captchaEnabled && !captchaToken)"
      >
        Sign up
      </b-button>
    </b-form>
    <b-modal
      id="2fa-modal"
      centered
      title="Two-Factor Authentication"
      @ok="handleOk"
      @cancel="resetModal"
    >
      <b-img
        center
        :src="totp.url"
        alt=""
      />
      <div class="form-input-group">
        <b-form-input
          id="totp_check_code"
          v-model="totp.code"
          required
          type="number"
          class="form-input"
          :state="isOtpValid()"
        />
        <label
          for="totp_check_code"
          class="form-label"
        ><span class="label-content">2FA code</span>
        </label>
        <b-form-text class="mt-0 pt-0">
          Please scan the QR code with your Google Authenticator App
        </b-form-text>
      </div>
      <template #modal-footer="{ ok }">
        <b-button
          class="px-4"
          pill
          size="sm"
          variant="success"
          @click="ok()"
        >
          Submit
        </b-button>
      </template>
    </b-modal>
  </div>
</template>

<script>

import { authenticator } from 'otplib';
import qrcode from 'qrcode';
import VueHcaptcha from '@hcaptcha/vue-hcaptcha';

export default {
  components: { VueHcaptcha },
  data() {
    return {
      captchaEnabled: this.$store.state.us_features.includes('reg_captcha'),
      captchaToken: null,
      totp: {
        url: '',
        code: '',
        secret: authenticator.generateSecret()
      },
      formData: {
        username: '',
        secret: '',
        name: '',
        id: '0'
      }
    };
  },
  methods: {
    onVerify(tok) {
      this.captchaToken = tok;
    },
    onExpired() {
      this.captchaToken = null;
    },
    isOtpValid() {
      return authenticator.check(this.totp.code, this.totp.secret);
    },
    set2faUrl() {
      qrcode.toDataURL(authenticator.keyuri(this.formData.username, 'toothpik.aerostun.dev', this.totp.secret), (err, imageUrl) => {
        this.totp.url = !err ? imageUrl : '';
      });
    },
    onSubmit(evt) {
      evt.preventDefault();
      if(this.captchaEnabled) {
        if(!this.captchaToken)
          return;
        this.formData.captcha_token = this.captchaToken;
      }
      this.$bvModal.show('2fa-modal');
    },
    resetModal() {
      this.formData.username = '';
      this.formData.secret = '';
    },
    handleOk(bvModalEvt) {
      bvModalEvt.preventDefault();
      if(!authenticator.check(this.totp.code, this.totp.secret))
        return;
      this.$store.dispatch('signup', { ...this.formData, secret: `${this.formData.secret}\x0A${this.totp.secret}` });
      this.$nextTick(() => {
        this.$bvModal.hide('2fa-modal');
      });
    }
  }
};
</script>

<style scoped>
input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
input[type=number] {
  -moz-appearance: textfield;
}
</style>
