import { createApp } from "vue";
import { createPinia } from "pinia";
import { router } from './router.ts';

import '$css/_index.css';

import App from "./app.tsx";

createApp(App)
    .use(createPinia())
    .use(router)
    .mount("#app");
