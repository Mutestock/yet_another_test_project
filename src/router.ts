import Home from "./components/Home.vue";
import Greet from "./components/Greet.vue";
import ConnectionSelection from "./components/ConnectionSelection.vue";
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
    {
        path: "/new-connection",
        name: "NewConnection",
        component: ConnectionSelection
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;
