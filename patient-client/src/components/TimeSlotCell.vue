<template>
  <b-container>
    <b-row>
      <b-col class="d-flex justify-content-between">
        <b-icon
          class="h3 ml-4 weeks-btn "
          icon="chevron-left"
          @click="previousWeek"
        />
        <b-icon
          class="h3 mr-4 weeks-btn"
          icon="chevron-right"
          @click="nextWeek"
        />
      </b-col>
    </b-row>
    <b-row
      id="timeslot-row"
      class="overflow-auto justify-content-md-center"
    >
      <b-col
        v-for="(s, index) in appSlots"
        :key="index"
        sm="4"
        md="2"
      >
        <p class="text-center py-3 date-text">
          {{ s.date }}
        </p>
        <hr>
        <b-row
          v-for="(appoint) in s.time"
          :key="appoint.time"
          class="my-2 mx-4"
        >
          <b-col
            :class="{ isAva: appoint.available, unAva: !appoint.available, selectedTime: formData.time === appoint.time && formData.date === s.date && appoint.available }"
            class="py-1 border text-center time-slot"
            @click="() => { formData = { date: s.date, time: appoint.time}; }"
          >
            <span> {{ appoint.time }}</span>
          </b-col>
        </b-row>
      </b-col>
    </b-row>
    <b-row>
      <b-col>
        <div class="my-3">
          <h3>Your Selected Time: {{ formData.date }} - {{ formData.time }}</h3>
        </div>
      </b-col>
    </b-row>
    <b-row>
      <b-col>
        <div class="mt-2 w-100 d-flex justify-content-around d-sm-block">
          <b-button
            type="submit"
            variant="primary"
            pill
            class="float-right px-2 ml-sm-5 px-sm-5 book-btn"
            @click="submitForm"
          >
            Quick Book
          </b-button>
          <b-button
            variant="primary"
            pill
            class="float-right px-2 px-sm-5 book-btn"
            @click="NormalForm"
          >
            Normal Book
          </b-button>
        </div>
      </b-col>
    </b-row>
  </b-container>
</template>

<script>

export default {
  name: 'MySlot',
  props: {
    appSlots: {
      type: Array,
      required: true,
      default() {
        return [];
      }
    }
  },
  data() {
    return {
      formData: {
        date: null,
        time: null
      }
    };
  },
  watch: {
    appSlots(val, old) {
      this.formData = {
        date: null,
        time: null
      };
    }
  },
  methods: {
    submitForm() {
      this.$store.state.bookedSlot = this.formData;
      this.$store.dispatch('quick_book', this.formData);
    },
    NormalForm() {
      this.$store.commit('booked_slot', this.formData);
      this.$store.commit('appointment');
    },
    previousWeek() {
      this.$store.dispatch('prev_week');
    },
    nextWeek() {
      this.$store.dispatch('next_week');
    }
  }
};
</script>

<style scoped>
.isAva {
  background: var(--white);
  cursor: pointer;
  box-shadow: 0.2rem 0.4rem 0.6rem rgba(0, 0, 0, 0.10) !important;
}
.isAva:hover {
  border: 1px solid #007bff !important;

}
.unAva {
  background: var(--white);
  opacity: 0.5;
  pointer-events: none;
  cursor: not-allowed;
  user-select: none;
}
.time-slot {
  font-size: 1.2rem;
  border-radius: 2px;
  transition: 0.3s;
  position: relative;

}
.selectedTime {
  border: 1px solid #007bff !important;
  position: relative;
}
#timeslot-row {
max-height: 50vh !important;
}
.date-text{
  position: -webkit-sticky;
  position: sticky;
  top: 0;
  z-index: 20;
  background: #ffffff;
}
.weeks-btn {
 cursor: pointer;
 color: skyblue;
}
</style>
