import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import { useAuthStore } from "./store/auth.ts";

import Home from "./views/home.tsx";
import { homeRouter } from "./views/home.router.ts";

import Login from "./views/login.tsx";

const routes: Array<RouteRecordRaw> = [
    {
        path: "/",
        name: "Home",
        component: Home,
        redirect: "/inbox",
        children: homeRouter,
    },
    {
        path: "/login",
        name: "Login",
        component: Login,
    },
];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});

router.beforeEach((to) => {
    const auth = useAuthStore();
    if (to.name !== "Login" && !auth.isAuthenticated) {
        return { name: "Login" };
    } else if (to.name === "Login" && auth.isAuthenticated) {
        return { name: "Home" };
    }
});
