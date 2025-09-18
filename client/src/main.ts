import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./app.tsx";

createApp(App)
    .use(createPinia())
    .mount("#app");
