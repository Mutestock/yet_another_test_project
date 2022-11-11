import Home from "./components/Home.vue";
import Greet from "./components/Greet.vue";
import { createRouter, RouteRecordRaw, createWebHistory} from "vue-router";

const routes: Array<RouteRecordRaw> = [
    {
        path: "/",
        name: "Home",
        component: Home
    },
    {
        path: "/greet",
        name: "Greet",
        component: Greet
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;
