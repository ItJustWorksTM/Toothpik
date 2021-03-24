<template>
  <b-row no-gutters>
    <b-col
      md="4"
      order="2"
      order-md="1"
      style="max-height: 60vh"
    >
      <div class="mx-2 mx-lg-5 mt-2 mb-2">
        <div class="position-sticky">
          <h5 class="text-center font-weight-bold">
            Find Dentist Near You
          </h5>
          <b-input-group>
            <b-form-input
              v-model="search"
              type="search"
              placeholder="Search Clinic"
            />
            <template #append>
              <b-input-group-text class="search-icon">
                <b-icon icon="search" />
              </b-input-group-text>
            </template>
          </b-input-group>
        </div>
        <div
          id="clinic-card-container"
          class="overflow-auto"
        >
          <ClinicCard
            :clinics="filteredClinic"
          />
        </div>
      </div>
    </b-col>
    <b-col
      md="8"
      order="1"
      order-md="2"
    >
      <l-map
        :zoom="zoom"
        :center="center"
        :options="mapOptions"
        style="min-height: 50vh"
        @update:center="center => (currentCenter = center)"
        @update:zoom="zoom => (currentZoom = zoom)"
      >
        <l-tile-layer
          :url="url"
          :attribution="attribution"
        />
        <l-marker
          v-for="(clinic, index) in filteredClinic"
          :key="clinic.id"
          :lat-lng="clinicCoordinate[index]"
        >
          <l-icon>
            <div v-if="$store.getters.is_week_available(clinic.id)">
              <img
                src="../assets/map_icon-1.png"
                alt="Ava"
              >
            </div>
            <div v-else>
              <img
                src="../assets/map_icon-2.png"
                alt="unAva"
              >
            </div>
          </l-icon>
          <l-popup>
            <div
              style="width: 200px; overflow: auto;"
            >
              <div>
                <span class="font-weight-bold">{{ clinic.name }}</span>
                <div><span class="font-weight-bold">By:</span> {{ clinic.owner }}</div>
                <div><span class="font-weight-bold">Address:</span>  {{ clinic.address }}</div>
              </div>
              <b-button
                class="px-5 center mt-3 ml-1 mb-1 float-right"
                size="sm"
                pill
                variant="primary"
                @click="bookClick(clinic.id, clinic.name)"
              >
                Book
              </b-button>
            </div>
          </l-popup>
        </l-marker>
      </l-map>
    </b-col>
  </b-row>
</template>

<script>
import { Icon, latLng } from 'leaflet';
import { LIcon, LMap, LMarker, LPopup, LTileLayer } from 'vue2-leaflet';
import ClinicCard from './ClinicCard';
import { mapState } from 'vuex';

/* eslint no-underscore-dangle: "off"*/
delete Icon.Default.prototype._getIconUrl;
Icon.Default.mergeOptions({
  iconRetinaUrl: require('leaflet/dist/images/marker-icon-2x.png'),
  iconUrl: require('leaflet/dist/images/marker-icon.png'),
  shadowUrl: require('leaflet/dist/images/marker-shadow.png')
});

export default {
  name: 'Map',
  components: {
    LMap,
    LTileLayer,
    LMarker,
    LPopup,
    ClinicCard,
    LIcon
  },
  data() {
    return {
      search: '',
      zoom: 13,
      center: latLng(57.70887, 11.97456),
      url: 'https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png',
      attribution:
        '&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors',
      currentZoom: 11.5,
      currentCenter: latLng(57.70887, 11.97456),
      mapOptions: {
        zoomSnap: 0.5
      }
    };
  },
  computed: {
    ...mapState([ 'dentists' ]),
    clinicCoordinate() {
      return this.filteredClinic.map(clinic => latLng(clinic.coordinate.latitude, clinic.coordinate.longitude));
    },
    filteredClinic() {
      return this.dentists.filter(clinic => clinic.name.toLowerCase().match(this.search.toLowerCase()));
    },
    isWeekAvailable() {
      const result = this.$store.getters.availability.is_week_available();
      console.log(result);
      return result;
    }
  },
  methods: {
    bookClick(id, name) {
      this.$store.state.clinicName = name;
      this.$store.commit('book', id);
    }
  }
};
</script>

<style>
.app-map-container {
  height: 50vh !important;
}
.search-icon {
  background: none !important;
}
#clinic-card-container::-webkit-scrollbar {
    width: 12px;
}

#clinic-card-container::-webkit-scrollbar-track {
    -webkit-box-shadow: inset 0 0 6px rgba(200,200,200,1);
    border-radius: 10px;
}

#clinic-card-container::-webkit-scrollbar-thumb {
    border-radius: 10px;
    background-color:#fff;
    -webkit-box-shadow: inset 0 0 6px rgba(90,90,90,0.7);
}
#clinic-card-container {
  max-height: 40vh;
}

</style>
