<template>
  <div class="about">
    <div id="signalingContainer" style="display: none">
      Browser base64 Session Description<br />
      <textarea id="localSessionDescription" readonly="true"></textarea> <br />

      Golang base64 Session Description<br />
      <textarea id="remoteSessionDescription"></textarea> <br />
      <button onclick="window.startSession()">Start Session</button><br />
    </div>

    <br />

    Video<br />
    <video id="video1" width="160" height="120" autoplay muted></video> <br />

    <button class="createSessionButton" @click="window.createSession(true)">
      Publish a Broadcast
    </button>
    <button class="createSessionButton" onclick="window.createSession(false)">
      Join a Broadcast</button
    ><br />

    <br />

    Logs<br />
    <div id="logs"></div>
  </div>
</template>

<script lang="ts">
import { Vue } from "vue-class-component";
import axios from "axios";
export default class Room extends Vue {
    private room_name: string = '';
    
     {
  let pc = new RTCPeerConnection({
    iceServers: [
      {
        urls: 'stun:stun.l.google.com:19302'
      }
    ]
  })
  //pc.oniceconnectionstatechange = e => log(pc.iceConnectionState)
  pc.onicecandidate = event => {
    if (event.candidate === null) {
      // @ts-ignore 
      document.getElementById('localSessionDescription').value = btoa(JSON.stringify(pc.localDescription))
    }
  }
  if (isPublisher) {
    navigator.mediaDevices.getUserMedia({ video: true, audio: false })
      .then(stream => {
        stream.getTracks().forEach(track => pc.addTrack(track, stream));
        document.getElementById('video1').srcObject = stream
        pc.createOffer()
          .then(d => pc.setLocalDescription(d))
      })
  } else {
    pc.addTransceiver('video')
    pc.createOffer()
      .then(d => pc.setLocalDescription(d))

    pc.ontrack = function (event) {
      var el = document.getElementById('video1')
      el.srcObject = event.streams[0]
      el.autoplay = true
      el.controls = true
    }
  }
}   
</script>
