<template>
  <b-container
    fluid
  >
    <b-row
      no-gutters
      class="mt-3"
    >
      <b-col
        sm="12"
        md="6"
        lg="4"
        xl="3"
        class="overflow-hidden app-img-col"
      >
        <b-img
          class="w-100 h-100 app-img"
          src="../assets/dental_operation.jpg"
          alt="image01"
        />
        <p class="font-weight-bold app-text">
          Dental Operation
        </p>
      </b-col>
      <b-col
        sm="0"
        md="6"
        lg="4"
        xl="3"
        class="overflow-hidden d-none d-md-inline app-img-col"
      >
        <b-img
          class="w-100 h-100 app-img"
          src="../assets/teeth_whitening.jpg"
          alt="image02"
        />
        <p class="font-weight-bold app-text">
          Tooth Whitening
        </p>
      </b-col>
      <b-col
        sm="0"
        lg="4"
        xl="3"
        class="overflow-hidden d-none d-lg-inline app-img-col"
      >
        <b-img
          class="w-100 h-100 app-img"
          src="../assets/teeth_care.jpg"
          alt="image01"
        />
        <p class="font-weight-bold app-text">
          Teeth Care
        </p>
      </b-col>
      <b-col
        sm="0"
        xl="3"
        class="overflow-hidden d-none d-xl-inline app-img-col"
      >
        <b-img
          class="w-100 h-100 app-img"
          src="../assets/dental_care.jpg"
          alt="image02"
        />
        <p class="font-weight-bold app-text">
          Dental Checkup
        </p>
      </b-col>
    </b-row>
    <b-row
      no-gutters
      class="mt-3 shadow"
    >
      <Map class="w-100 h-100" />
    </b-row>
  </b-container>
</template>

<script>
import Map from '../components/Map.vue';

export default {
  name: 'Home',
  components: {
    Map
  }
};
</script>

<style scoped>
.app-img{
  object-fit: cover;
    transition: transform 750ms cubic-bezier(0.5, 0, 0.5, 1),
                opacity 250ms linear;
}
.app-img:hover {
  transform: scale(1.2);
  opacity: 0.75;
}
.app-img-col {
  background: var(--teal);
}
.app-text {
  position: absolute;
  z-index: 10;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
  font-size: 1.25rem;
}
@media screen and (max-width:769px){
  .app-img {
    height: 40vh !important;
  }
}
</style>
