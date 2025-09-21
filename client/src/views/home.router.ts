import { RouteRecordRaw } from "vue-router";

import Inbox from "./home/inbox.tsx";

export const homeRouter: RouteRecordRaw[] = [
    { path: "", redirect: "/inbox" },
    { path: "/inbox", name: "Inbox", component: Inbox },
];
