import { defineComponent } from 'vue';

import InboxView from '../../widgets/inbox/view.tsx';

export default defineComponent({
    name: 'InboxView',
    setup() {
        return () => (
            <div>
                <InboxView />
            </div>
        )
    }
})
