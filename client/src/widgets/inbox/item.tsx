import { defineComponent } from "vue";
import { useInboxStore } from "../../store/inbox.ts";
import { Inbox } from "$pmodel/inbox.ts";
import { ref } from "../../infra/ref.ts";

type TypeData = keyof Omit<Inbox, "id">;
type DisplayData = { label: string; value: string | undefined; type: TypeData };
function mapInboxItemToDisplay(item: Inbox): DisplayData[] {
    const displayData: DisplayData[] = [];
    displayData.push({ label: "Title", value: item.title, type: "title" });
    displayData.push({ label: "Artist", value: item.artist, type: "artist" });
    displayData.push({ label: "Album", value: item.album, type: "album" });
    displayData.push({ label: "File", value: item.file, type: "file" });
    displayData.push({
        label: "Extra Info",
        value: item.extra_info,
        type: "extra_info",
    });
    return displayData;
}

export default defineComponent({
    name: "InboxList",
    props: {
        item: {
            type: Object as () => Inbox,
            required: true,
        },
    },
    setup(props) {
        const inboxStore = useInboxStore();
        const editMode = ref(false);
        const displayData = mapInboxItemToDisplay(props.item);

        function deleteItem() {
            inboxStore.deleteInboxItem(props.item.id);
        }

        function saveItem(item: DisplayData[]) {
            const inbox: Inbox = {
                id: props.item.id,
            };

            item.forEach((data) => {
                inbox[data.type] = data.value;
            });

            inboxStore.updateInboxItem(inbox);
            editMode.value = false;
        }

        return () => (
            <div data-widget="inbox-item">
                {displayData.map((data) => (
                    (editMode.value || data.value) && (
                        <div class="row">
                            <span class="label">{data.label}:</span>
                            {editMode.value
                                ? <input type="text" value={data.value} />
                                : <span class="value">{data.value}</span>}
                        </div>
                    )
                ))}
                {displayData.length === 0 && (
                    <div class="empty">No details available.</div>
                )}
                {displayData.length > 0 && (
                    <div class="actions">
                        {editMode.value
                            ? (
                                <button type="button" onClick={() => saveItem(displayData)}>
                                    Save
                                </button>
                            )
                            : (
                                <button type="button" onClick={deleteItem}>
                                    Delete
                                </button>
                            )}
                        {editMode.value
                            ? (
                                <button
                                    type="button"
                                    onClick={() => {
                                        editMode.value = false;
                                    }}
                                >
                                    Cancel
                                </button>
                            )
                            : (
                                <button
                                    type="button"
                                    onClick={() => {
                                        editMode.value = true;
                                    }}
                                >
                                    Edit
                                </button>
                            )}
                    </div>
                )}
            </div>
        );
    },
});
