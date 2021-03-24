
<template>
  <div>
    <b-card class="shadow-sm">
      <div class="d-flex justify-content-between">
        <div class="border-right border-dark d-inline-block book-date">
          <b-card-text>Nev</b-card-text>
          <b-card-text class="font-weight-bold book-day text-danger">
            18
          </b-card-text>
          <b-card-text>Wen</b-card-text>
        </div>
        <div class="d-inline-block content-container">
          <div class="booking-time">
            <p>Time</p>
            <p class="font-weight-bold">
              18:55
            </p>
          </div>
          <hr>
          <div class="d-flex justify-content-between">
            <div class="booking-type">
              <p>Appointment Type</p>
              <p class="font-weight-bold">
                teeth Check up
              </p>
            </div>
            <div class="d-none d-md-block">
              <b-button class="booking-btn text-dark mx-2">
                Edit
              </b-button>
              <b-button class="booking-btn text-danger ml-2">
                Cancel
              </b-button>
            </div>
          </div>
        </div>
      </div>
    </b-card>
  </div>
</template>

<script>
export default {
};
</script>

<style scoped>
.book-day{
  font-size: 1.5rem;
}
.book-date{
  width: 15%;
}
.content-container{
  width: 80%;
}
.booking-type{
  line-height: 0.5rem;
}
.booking-time{
  line-height: 0.5rem;
}
.booking-btn{
  background: none;
  border: none;
  font-weight: bold;
}
</style>
