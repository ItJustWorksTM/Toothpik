<template>
  <footer class="w-100 mt-4 text-center text-muted">
    &copy; {{ year }} <span
      class="font-weight-bold ml-1 text-primary"
    >Toothpik™️</span>
  </footer>
</template>
<script>
export default {
  data() {
    return {
      year: new Date().getFullYear()
    };
  }
};
</script>
