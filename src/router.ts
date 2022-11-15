import Home from "./components/Home.vue";
import ConnectionSelection from "./components/ConnectionSelection.vue";
import { createRouter, RouteRecordRaw, createWebHistory} from "vue-router";

const routes: Array<RouteRecordRaw> = [
    {
        path: "/",
        name: "Home",
        component: Home
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
