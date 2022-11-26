import Home from "./components/Home.vue";
import ConnectionSelection from "./components/ConnectionSelection.vue";
import MongoConnection from "./components/connection/MongoConnection.vue";
import PostgresConnection from "./components/connection/PostgresConnection.vue";
import RedisConnection from "./components/connection/RedisConnection.vue";
import { createRouter, RouteRecordRaw, createWebHistory, Router, RouteRecordNormalized } from "vue-router";
import { backendLog } from "./utils/logging";

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
    },
    {
        path: "/new_mongo",
        name: "MongoConnection",
        component: MongoConnection
    },
    {
        path: "/new_postgres",
        name: "PostgresConnection",
        component: PostgresConnection
    },
    {
        path: "/new_redis",
        name: "RedisConnection",
        component: RedisConnection
    },
];

const router: Router = createRouter({
    history: createWebHistory(),
    routes
});

const directions: Map<string, string> = new Map();
directions.set("new_connection", "NewConnection");
directions.set("postgres", "PostgresConnection");
directions.set("redis", "RedisConnection");
directions.set("mongo", "MongoConnection");


export async function handleRedirect(eventName: string): Promise<void> {
    let redirectionName = directions.get(eventName)
    if (redirectionName) {
        await redirectByEventName(redirectionName)
    }
    else {
        backendLog(`Front=>reroute in router pointed to invalid route: ${eventName}`, "error")
    }
}


async function redirectByEventName(eventName: string): Promise<void> {
    let pathRaw: RouteRecordNormalized | undefined = router
        .getRoutes()
        .find(x => x.name == eventName)
    if (pathRaw !== undefined) {
        router.push(pathRaw)
    }
}



export default router;
