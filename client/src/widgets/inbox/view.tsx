import { defineComponent } from 'vue';
import { useInboxStore } from "../../store/inbox.ts";
import { triggerEvent } from '$infra/events.ts';

import InboxItem from "./item.tsx";

export default defineComponent({
    name: 'InboxList',
    setup() {
        const inboxStore = useInboxStore()
        inboxStore.loadInbox()

        function addNewItem() {
            triggerEvent("dsot:inbox:new", {});
        }

        return () => (
            <div data-widget="inbox-view">
                <button type="button" onClick={addNewItem}>Add new</button>
                <div class="items">
                    {inboxStore.values.map(item => (
                        <InboxItem key={item.id} item={item} />
                    ))}
                </div>
            </div>
        )
    }
})
