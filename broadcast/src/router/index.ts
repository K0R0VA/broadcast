import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";
import Home from "../views/Home.vue";
import Room from "../views/Room.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/room/:room_name",
    name: "About",
    component: Room
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
