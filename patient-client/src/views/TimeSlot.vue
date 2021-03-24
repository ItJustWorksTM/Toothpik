<template>
  <b-container class="mb-5">
    <h3 class="my-3">
      Your Clinic of choice: <span class="text-info">{{ this.$store.state.clinicName }}</span>
    </h3>
    <h3 class="my-3">
      Please pick a slot that suits you
    </h3>
    <div class="mt-6 availability-slots-container shadow">
      <br>
      <TimeSlotCell
        :app-slots="this.$store.state.availability.availability[this.$store.state.appointment.dentistid]"
      />
      <br>
    </div>
  </b-container>
</template>

<script>
import TimeSlotCell from '../components/TimeSlotCell.vue';
export default {
  components: {
    TimeSlotCell
  },
  data() {
    return {
      dateNow: new Date(),
      fields: [
        {
          key: 'startTime'
        }
      ]
    };
  }
};
</script>
<style scoped>
.availability-slots-container{
  margin: auto;
}

</style>
