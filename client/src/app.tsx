import { defineComponent } from 'vue';
import { RouterView } from 'vue-router';
import { useInboxStore } from './store/inbox.ts';
import { onDsotEvent } from '$infra/events.ts';

export default defineComponent({
    name: 'App',
    setup() {
        const inboxStore = useInboxStore();

        onDsotEvent("dsot:inbox:new", (event) => {
            inboxStore.nemEmptyInboxItem(event.detail.value);
        });

        return () => (
            <RouterView />
        )
    }
})
