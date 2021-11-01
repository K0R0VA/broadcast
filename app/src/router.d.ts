import VueRouter from "vue-router";
import { Route } from "vue-router";

declare module "@vue/runtime-core" {
  export interface ComponentCustomProperties {
    $router: VueRouter;
    $route: Route;
  }
}
