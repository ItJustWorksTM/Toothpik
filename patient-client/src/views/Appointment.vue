<template>
  <div class="outer-container">
    <div class="booking-container">
      <p class="font-weight-bold mx-4 mb-2 booking-text">
        Book new Appointment
      </p>
      <div class="m-auto booking-form">
        <b-form @submit="onSubmit">
          <b-row
            cols="1"
            cols-md="2"
            cols-lg="3"
            align-h="around"
          >
            <b-col class="my-2">
              <b-form-input
                v-model="formData.name"
                placeholder="Name"
              />
            </b-col>
            <b-col class="my-2">
              <b-form-datepicker
                v-model="DOB"
                start-weekday="1"
                placeholder="Date of Birth"
                locale="en"
              />
            </b-col>
            <b-col class="my-2">
              <b-form-input
                v-model="formData.mobile"
                placeholder="Mobile Number"
              />
            </b-col>
            <b-col class="my-2">
              <b-form-input
                v-model="formData.email"
                placeholder="Email Address"
              />
            </b-col>
            <b-col class="my-2">
              <b-form-select
                id="service"
                v-model="service"
                :options="serviceOptions"
              >
                <template #first>
                  <b-form-select-option
                    :value="null"
                    disabled
                  >
                    Please select a Service
                  </b-form-select-option>
                </template>
              </b-form-select>
            </b-col>
            <b-col class="my-2">
              <b-form-input
                v-model="formData.reason"
                placeholder="Condition"
              />
            </b-col>
          </b-row>
          <b-row
            cols="1"
            cols-md="2"
            cols-lg="3"
          />
          <b-row>
            <b-col sm="12">
              <b-button
                pill
                variant="primary"
                class="px-5 mb-3 float-right"
                type="submit"
              >
                Save
              </b-button>
            </b-col>
          </b-row>
        </b-form>
      </div>
    </div>
    <div class="mt-5 booking-cards-container position-relative">
      <div class="m-auto booking-form">
        <b-row
          cols="1"
          cols-lg="2"
        >
          <b-col class="mb-4 mb-md-0">
            <p class="font-weight-bold booking-text">
              Upcoming
            </p><BookingCard /> <BookingCard class="mt-3" />
          </b-col>
          <b-col>
            <p class="font-weight-bold booking-text ">
              Past
            </p><BookingCard /> <BookingCard class="mt-3 mb-5" />
          </b-col>
        </b-row>
      </div>
      <div class="temp-container w-100 h-100 position-absolute" />
      <h1 class="position-absolute comming-soon-text text-center">
        Comming Soon
      </h1>
    </div>
  </div>
</template>

<script>
import BookingCard from '../components/BookingCard.vue';

export default {
  name: 'Appointment',
  components: {
    BookingCard
  },
  data() {
    return {
      serviceOptions: [
        { value: 'a', text: 'Tooth Whitening' },
        { value: 'b', text: 'Tooth Surgery' },
        { value: 'c', text: 'Dental Operation' },
        { value: 'd', text: 'General Check up' }
      ],
      value: '',
      DOB: '',
      service: null,
      formData: {
        reason: '',
        mobile: '',
        email: '',
        name: ''
      }
    };
  },
  methods: {
    onSubmit(evt) {
      evt.preventDefault();
      this.$store.dispatch('custom_book', this.formData);
    }
  }
};
</script>

<style scoped>
.outer-container{
  margin: auto;
  width: 90%;
  height: 100%;
}
.booking-container {
    background: var(--pink);
}
.booking-form{
  width: 95%;
}
.booking-cards-container{
  background: var(--light);
}
.booking-text{
  line-height: 3rem;
}
.temp-container {
  background: var(--white);
  opacity: 0.8;
  top: 0;
  left: 0;
}
.comming-soon-text {
  top: 50%;
  left: 50%;
  transform: translateX(-50%);
}
</style>
