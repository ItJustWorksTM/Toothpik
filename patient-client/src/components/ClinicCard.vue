<template>
  <div>
    <div
      v-for="clinic in clinics"
      :key="clinic.id"
      class="cards mt-2"
    >
      <div class="mt-2 py-1 pl-4">
        <div class="font-weight-bold">
          {{ clinic.name }}
        </div>
        <div>{{ clinic.address }}</div>
        <div><strong>Available slot: </strong> {{ $store.getters.is_week_available(clinic.id) ? 'Yes' : 'No' }} </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    clinics: {
      type: Array,
      default: () => []
    }
  }
};
</script>

<style scoped>
.cards:nth-child(odd) {
  background: var(--light-gray);
  border-radius: 4px;
}
</style>
