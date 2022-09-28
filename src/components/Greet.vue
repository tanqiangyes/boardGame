<script>
import { invoke } from "@tauri-apps/api/tauri";

export default {
  name: "greet",
  data() {
    return {
      num: 0,
      plates: [],
    }
  },
  methods: {
    greet: async function() {
      this.plates = [];
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      this.plates = await invoke("greet",{ num: this.num })
          .then((message) => {
            return message;
          })
          .catch((error) => {
            alert(error);
            console.error(error);
            return [];
          });
    }
  }
}
</script>

<script setup>
import Bull from "./Bull.vue";
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model.number="num" placeholder="Enter a number of plate..." />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <p id="greet-msg">
    <Bull :data="plates"></Bull>
  </p>
</template>
