import { defineComponent } from 'vue';
import { Menu } from 'lucide-vue-next';

import { useMenuStore } from "../store/menu.ts";

export default defineComponent({
    name: 'MenuToggle',
    setup() {
        const menuStore = useMenuStore();

        function toggleMenu() {
            menuStore.toggle();
        }

        return () => (
            <div data-widget="menu-toggle" onClick={toggleMenu}>
                <Menu />
            </div>
        )
    }
});
