import { defineComponent } from 'vue';
import { useInboxStore } from "../store/inbox.ts";

export default defineComponent({
    name: 'InboxList',
    setup() {
        const inboxStore = useInboxStore()
        inboxStore.loadInbox()

        return () => (
            <table>
                <thead>
                    <tr>
                        <th>Title</th>
                        <th>Artist</th>
                        <th>Album</th>
                        <th>File</th>
                        <th>Extra Info</th>
                    </tr>
                </thead>
                <tbody>
                    {inboxStore.values.map(item => (
                        <tr key={item.id}>
                            <td>{item.title || ''}</td>
                            <td>{item.artist || ''}</td>
                            <td>{item.album || ''}</td>
                            <td>{item.file || ''}</td>
                            <td>{item.extra_info || ''}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
        )
    }
})
