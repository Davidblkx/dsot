import { defineComponent } from 'vue';
import { useInboxStore } from "../../store/inbox.ts";
import { Inbox } from "$pmodel/inbox.ts";

function mapInboxItemToDisplay(item: Inbox): { label: string; value: string }[] {
    const displayData = [];
    if (item.title) {
        displayData.push({ label: 'Title', value: item.title });
    }
    if (item.artist) {
        displayData.push({ label: 'Artist', value: item.artist });
    }
    if (item.album) {
        displayData.push({ label: 'Album', value: item.album });
    }
    if (item.file) {
        displayData.push({ label: 'File', value: item.file });
    }
    if (item.extra_info) {
        displayData.push({ label: 'Extra Info', value: item.extra_info });
    }
    return displayData;
}

export default defineComponent({
    name: 'InboxList',
    props: {
        item: {
            type: Object as () => Inbox,
            required: true,
        }
    },
    setup(props) {
        const inboxStore = useInboxStore()
        const displayData = mapInboxItemToDisplay(props.item);

        return () => (
            <div data-widget="inbox-item">
                {displayData.map(data => (
                    <div class="row">
                        <span class="label">{data.label}</span>
                        <span class="value">{data.value}</span>
                    </div>
                ))}
                {
                    displayData.length === 0 && <div class="empty">No details available.</div>
                }
            </div>
        )
    }
});
