import { defineComponent } from 'vue';

import InboxList from '../../widgets/inbox_list.tsx';

export default defineComponent({
    name: 'InboxView',
    setup() {
        return () => (
            <div>
                INBOX
                <InboxList />
            </div>
        )
    }
})
