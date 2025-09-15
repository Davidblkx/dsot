import { defineComponent, ref, type Ref } from 'vue';
import { core } from "$platform";

export default defineComponent({
    name: 'App',
    setup() {
        const users: Ref<{ id: string, name: string }[]> = ref([]);

        async function onClick() {
            const req = await core.executeCommand('users-list', {});
            if (req.success) {
                users.value = req.value;
            }
        }

        return () => (
            <div>
                <h1>Users</h1>
                <button type="button" onClick={onClick}>Add User</button>
                <ul>
                    {users.value.map(user => (
                        <li key={user.id}>{user.name}</li>
                    ))}
                </ul>
            </div>
        )
    }
})
