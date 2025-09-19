import { defineComponent } from 'vue';
import { RouterView } from 'vue-router';

import layoutStyles from '$css/layout.module.css';

import UserAvatar from "../widgets/current_user_avatar.tsx";

export default defineComponent({
    name: 'HomeView',
    setup() {
        return () => (
            <div class={layoutStyles.view}>
                <header class="header"><UserAvatar /></header>
                <div class="main">
                    <RouterView />
                </div>
                <footer class="footer"></footer>
            </div>
        )
    }
})
