import { defineComponent, computed } from 'vue';
import { Avatar } from '@ark-ui/vue';
import { useAuthStore } from "../store/auth.ts";
import { useRouter } from "vue-router";

export default defineComponent({
    name: 'CurrentUserAvatar',
    setup() {
        const authStore = useAuthStore()
        const router = useRouter()

        const userTwoLetterName = computed(() => authStore.user.name.slice(0, 2).toUpperCase())

        function logout() {
            authStore.logout()
            router.push('/login')
        }

        return () => (
            <Avatar.Root onClick={logout}>
                <Avatar.Fallback>{userTwoLetterName.value}</Avatar.Fallback>
            </Avatar.Root>
        )
    }
})
