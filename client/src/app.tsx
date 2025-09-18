import { defineComponent } from 'vue';
import { useAuthStore } from "./store/auth.ts";

export default defineComponent({
    name: 'App',
    setup() {
        const auth = useAuthStore();
        auth.loadUsers();

        return () => (
            <div class="main-container">
                <header class="header">{auth.user.name}</header>
                <div class="main">

                </div>
                <footer class="footer"></footer>
            </div>
        )
    }
})
