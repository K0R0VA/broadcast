<template>
  <div>
    <h2>Create room, share it and speek with your friends</h2>
    <input v-model="room_name" type="text" />
    <button @click="createRoom()"/>
  </div>
</template>

<script lang="ts">
import { Vue } from "vue-class-component";
import axios from "axios";
export default class Home extends Vue {
    private room_name: string = '';
    private async createRoom() {
        const _ = await axios.post('http://localhost:8081/create-room', this.room_name);
        // @ts-ignore
        this.$router.push({name: 'room', params: {room_name: this.room_name}});    
    }
}   
</script>