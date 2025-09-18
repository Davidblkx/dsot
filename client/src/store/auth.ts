import { defineStore } from "pinia";
import { computed } from "vue";
import { ref } from "$infra/ref.ts";
import { User } from "$pmodel/user.ts";
import { core } from "$platform";
import { createLocalStorageStore } from "./plugins/local_storage.ts";

export const useAuthStore = defineStore("auth", () => {
    const storage = createLocalStorageStore<User>({ key: "auth.user", defaultValue: { id: "-1", name: "unknown" } });
    const user = ref(storage.load());
    const users = ref<User[]>([]);

    const isAuthenticated = computed(() => user.value.id !== "-1");

    async function loadUsers(force = false) {
        if (!force && users.value.length > 0) {
            return;
        }

        const req = await core.executeCommand('users-list', {});
        if (req.success) {
            users.value = req.value;
        } else {
            console.error("Failed to load users:", req.error);
        }
    }

    async function login(userId: string) {
        await loadUsers();
        const foundUser = users.value.find(u => u.id === userId);
        if (foundUser) {
            user.value = foundUser;
            storage.save(user.value);
        } else {
            throw new Error("User not found");
        }
    }

    function logout() {
        user.value = { id: "-1", name: "unknown" };
        storage.save(user.value);
    }

    return { user, users, isAuthenticated, loadUsers, login, logout };
});
