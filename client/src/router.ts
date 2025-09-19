import { createRouter, RouteRecordRaw, createWebHistory } from 'vue-router'
import { useAuthStore } from "./store/auth.ts";

import Home from './views/home.tsx'
import Inbox from './views/inbox.tsx'
import Login from './views/login.tsx'

const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        name: 'Home',
        component: Home,
        redirect: '/inbox',
        children: [
            { path: '', redirect: '/inbox' },
            { path: '/inbox', name: 'Inbox', component: Inbox },
        ],
    },
    {
        path: '/login',
        name: 'Login',
        component: Login
    }
]

export const router = createRouter({
    history: createWebHistory(),
    routes,
})

router.beforeEach((to) => {
    const auth = useAuthStore();
    if (to.name !== 'Login' && !auth.isAuthenticated) {
        return { name: 'Login' }
    } else if (to.name === 'Login' && auth.isAuthenticated) {
        return { name: 'Home' }
    }
});
