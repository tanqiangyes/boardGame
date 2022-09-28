<template>
  <div class="row" v-if="data.length" v-for="(bull, index) in data" v-bind:id="'bull'+ index">
    <div  v-for="(plate, bindex) in bull" v-bind:id="'plate'+ bindex">
<!--      {{plate.pcolor}}{{plate.pvalue}}-->
      <img v-bind:src="'/pokers/'+plate.pcolor+plate.pvalue+'.jpg'" alt="">
    </div>
    <button class="button" v-bind:id="'button'+ index" type="button" @click="cal(index)">cal</button>
  </div>
</template>

<script>
import {invoke} from "@tauri-apps/api/tauri";
export default {
  name: "bull",
  props: {
    data: {
      type: Array,
      default: () => [],
    },
  },
  data() {
    return {
      // comment: "cals",
    }
  },
  methods: {
    cal: async function (index) {
      console.log("---------")
      console.log(index)
      console.log("---------")
      let item = document.getElementById("button"+ index);
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      item.innerText = await invoke("cal", {bull: this.data[index]})
          .then((message) => {
            return message;
          })
          .catch((error) => {
            alert(error);
            console.error(error);
            return false;
          });
    },
  },
}
</script>
