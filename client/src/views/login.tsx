import { defineComponent } from 'vue';
import { useAuthStore } from '../store/auth.ts';
import { ref } from "$infra/ref.ts";
import { useRouter } from "vue-router";

import layoutStyles from '$css/layout.module.css';
import textStyles from '$css/text.module.css';

import Section from '../components/section.tsx';

export default defineComponent({
    name: 'LoginView',
    setup() {
        const hasUsers = ref(false);
        const authStore = useAuthStore();
        const router = useRouter();

        authStore.loadUsers().then(() => {
            hasUsers.value = true;
        });

        async function loginUser(userId: string) {
            await authStore.login(userId);
            router.push({ name: 'Home' });
        }

        return () => (
            <div class={layoutStyles.viewCenter}>
                {
                    !hasUsers.value && <div>Loading...</div>
                }
                {
                    hasUsers.value && authStore.users.length === 0 && <div>No users found. Please set up an admin user.</div>
                }
                {
                    hasUsers.value && authStore.users.length > 0 && (
                        <Section>
                            {{
                                title: () => <div>Login</div>,
                                default: () => <div class={layoutStyles.content} style={{ minWidth: '30vw', minHeight: '20vh' }}>
                                    {authStore.users.map(user => (
                                        <button type="button" onClick={() => loginUser(user.id)}>{user.name}</button>
                                    ))}
                                </div>,
                                footer: () => <div class={textStyles.subtitle}>Pick your user</div>
                            }}
                        </Section>
                    )
                }
            </div>
        )
    }
})
