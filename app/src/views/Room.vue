<template>
  <div>
    <h1>{{ roomName }}</h1>
    <div>
      <div v-for="({ id }, index) in users.concat({ id: 'self' })" :key="index">
        <video :id="id" width="500" height="500" autoplay muted />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Vue } from "vue-class-component";
import { InjectReactive } from "vue-property-decorator";
Vue.registerHooks(["beforeRouteEnter", "beforeRouteLeave"]);

export default class Room extends Vue {
  private usersIds: string[] = [];
  private roomName = "";
  private sessions: Map<string, string> = new Map();
  private peer_connections: Map<string, RTCPeerConnection> = new Map();

  @InjectReactive()
  private websocket: WebSocket;

  beforeRouteEnter(
    to: { params: { roomId: string } },
    _: unknown,
    next: (arg: (self: Room) => Promise<void>) => void
  ) {
    next(async (self: Room) => {
      const response = await fetch(
        `http://127.0.0.1:8081/room/${to.params.roomId}/name`
      );
      const name = await response.json();
      self.roomName = name;
    });
  }

  beforeRouteLeave(_to: unknown, _from: unknown, next: () => void) {
    this.websocket.send(JSON.stringify({ type: "LeaveTheRoom", data: "" }));
    next();
  }

  mounted() {
    this.websocket.send(
      JSON.stringify({ type: "EnterTheRoom", data: this.roomId })
    );
    this.websocket.addEventListener("message", (e) => {
      const event = JSON.parse(e.data);
      switch (event.type) {
        case "RoomSessions": {
          this.usersIds = event.data;
          this.usersIds.forEach((userId) => {
            this.startRecive(userId);
          });
          break;
        }
        case "BroadcastDescription": {
          this.startSession("self", event.data);
          break;
        }
        case "NewSession": {
          console.log("asd");
          this.usersIds.push(event.data);
          this.startRecive(event.data);
          break;
        }
        case "RecipientDescription": {
          const { broadcaster_id, description } = event.data;
          this.startSession(broadcaster_id, description);
          break;
        }
      }
    });
    this.startBroadcasting();
  }

  private createConnection(userId: string): RTCPeerConnection {
    let pc = new RTCPeerConnection({
      iceServers: [
        {
          urls: [
            // "stun:stun.l.google.com:19302",
            // "stun:stun01.sipphone.com",
            // "stun:stun.ekiga.net",
            // "stun:stun.fwdnet.net",
            // "stun:stun.ideasip.com",
            // "stun:stun.iptel.org",
            // "stun:stun.rixtelecom.se",
            // "stun:stun.schlund.de",
            "stun:stun.l.google.com:19302",
            // "stun:stun1.l.google.com:19302",
            // "stun:stun2.l.google.com:19302",
            // "stun:stun3.l.google.com:19302",
            // "stun:stun4.l.google.com:19302",
            // "stun:stunserver.org",
            // "stun:stun.softjoys.com",
            // "stun:stun.voiparound.com",
            // "stun:stun.voipbuster.com",
            // "stun:stun.voipstunt.com",
            // "stun:stun.voxgratia.org",
            // "stun:stun.xten.com",
          ],
        },
      ],
    });
    pc.oniceconnectionstatechange = () => console.log(pc.iceConnectionState);
    pc.onicecandidate = (event) => {
      if (event.candidate === null) {
        this.websocket.send(
          JSON.stringify({
            type: "StartBroadcast",
            data: btoa(JSON.stringify(pc.localDescription)),
          })
        );
      }
    };
    return pc;
  }

  private async startBroadcasting() {
    let pc = this.createConnection("self");
    const stream = await navigator.mediaDevices.getUserMedia({
      video: true,
      audio: false,
    });
    stream.getTracks().forEach((track) => pc.addTrack(track, stream));
    // @ts-ignore
    document.getElementById("self").srcObject = stream;
    const description = await pc.createOffer();
    await pc.setLocalDescription(description);
    this.peer_connections.set("self", pc);
  }

  private startSession(userId: string, description: string) {
    this.peer_connections
      .get(userId)
      .setRemoteDescription(
        new RTCSessionDescription(JSON.parse(atob(description)))
      );
  }

  private async startRecive(userId: string) {
    let pc = this.createConnection(userId);
    pc.addTransceiver("video");
    const description = await pc.createOffer();
    await pc.setLocalDescription(description);
    const local_description = this.sessions.get(userId);
    this.peer_connections.set(userId, pc);
    this.websocket.send(
      JSON.stringify({ type: "StartBroadcast", data: local_description })
    );
    pc.ontrack = function (event) {
      var el = document.getElementById(userId);
      // @ts-ignore
      el.srcObject = event.streams[0];
      // @ts-ignore
      el.autoplay = true;
      // @ts-ignore
      el.controls = true;
    };
  }

  get users() {
    return this.usersIds.map((id) => {
      return { id };
    });
  }
  get roomId() {
    return this.$route.params.roomId;
  }
}
</script>
