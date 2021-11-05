<template>
  <div>
    <h1>Создайте свою комнату</h1>
    <input v-model="roomName" placeholder="введите имя комнаты" />
    <button @click="createRoom">Создать</button>
  </div>
</template>

<script lang="ts">
import { Vue } from "vue-class-component";

export default class Home extends Vue {
  private roomName = "";
  private async createRoom() {
    const response = await fetch("http://127.0.0.1:8081/create-room", {
      body: this.roomName,
      method: "POST",
    });
    const id = await response.json();
    this.$router.push({ name: "Room", params: { roomId: id } });
  }
}
</script>
