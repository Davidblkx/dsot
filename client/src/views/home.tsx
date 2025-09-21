import { defineComponent } from 'vue';
import { RouterView } from 'vue-router';

import { useMenuStore } from "../store/menu.ts";
import UserAvatar from "../widgets/current_user_avatar.tsx";
import MenuToggle from "../widgets/menu_toggle.tsx";

export default defineComponent({
    name: 'HomeView',
    setup() {
        const menuStore = useMenuStore();

        return () => (
            <div data-view-layout="home" class={menuStore.isOpenClass}>
                <header>
                    <div class="left">
                        <MenuToggle />
                    </div>
                    <div class="center"></div>
                    <div class="right">
                        <UserAvatar class="user" />
                    </div>
                </header>
                <div class="menu">
                </div>
                <div class="content">
                    <RouterView />
                </div>
                <footer></footer>
            </div>
        )
    }
})
