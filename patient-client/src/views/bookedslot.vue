// just for demo
<template>
  <b-container class="time-container shadow-lg">
    <div class="inner-container">
      <h1>You Have Booked:</h1>
      <h4>ON: <span class="text-info">{{ this.$store.state.clinicName }}</span></h4>
      <h4>At: <span class="text-info">{{ this.$store.state.bookedSlot.date }} - {{ this.$store.state.bookedSlot.time }}</span></h4>
      <b-link
        class="btn btn-primary mt-5 float-right"
        to="/"
      >
        Back to Home
      </b-link>
    </div>
  </b-container>
</template>

<script>
export default {
};
</script>

<style scoped>
.time-container {
  height: 70vh;
  position: relative;
}
.inner-container {
  position: absolute;
  top: 20%;
  left: 50%;
  transform: translatex(-50%);
}
</style>
