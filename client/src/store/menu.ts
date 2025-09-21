import { defineStore } from "pinia";
import { computed } from "vue";
import { ref } from "$infra/ref.ts";
import { createTypedStore } from "$infra/store.ts";
import { createLocalStorageStore } from "./plugins/local_storage.ts";

export const useMenuStore = createTypedStore(defineStore("menu", () => {
    const isOpenStorage = createLocalStorageStore<boolean>({
        key: "menu.isOpen",
        defaultValue: true,
    });
    const isOpen = ref(isOpenStorage.load());

    const isOpenClass = computed(() =>
        isOpen.value ? "menu_open" : "menu_closed"
    );

    function toggle() {
        isOpen.value = !isOpen.value;
        isOpenStorage.save(isOpen.value);
    }

    return {
        isOpen,
        isOpenClass,
        toggle,
    };
}));
