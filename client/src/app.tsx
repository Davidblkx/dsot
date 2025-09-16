import { defineComponent, ref, type Ref } from 'vue';
import { core } from "$platform";
import { User } from "$pmodel/user.ts";
import styles from '$css/layout.module.css';

export default defineComponent({
    name: 'App',
    setup() {
        const users: Ref<User[]> = ref([]);

        async function onClick() {
            const req = await core.executeCommand('users-list', {});
            if (req.success) {
                users.value = req.value;
            }
        }

        return () => (
            <div class={styles.layout}>
                <h1>Users</h1>
                <button class={styles.red} type="button" onClick={onClick}>Add User</button>
                <ul>
                    {users.value.map(user => (
                        <li key={user.id}>{user.name}</li>
                    ))}
                </ul>
            </div>
        )
    }
})
