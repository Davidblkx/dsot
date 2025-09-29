import { defineComponent } from 'vue';
import { useInboxStore } from "../../store/inbox.ts";

import InboxItem from "./item.tsx";

export default defineComponent({
    name: 'InboxList',
    setup() {
        const inboxStore = useInboxStore()
        inboxStore.loadInbox()

        return () => (
            <div data-widget="inbox-view">
                <div class="items">
                    {inboxStore.values.map(item => (
                        <InboxItem key={item.id} item={item} />
                    ))}
                </div>
            </div>
        )
    }
})
