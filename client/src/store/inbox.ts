import { defineStore } from "pinia";
import { ref } from "$infra/ref.ts";
import { createTypedStore } from "$infra/store.ts";
import { Inbox } from "$pmodel/inbox.ts";
import { core } from "$platform";
import { createLocalStorageStore } from "./plugins/local_storage.ts";

type StorageState = {
    limit: number;
    offset: number;
}

export const useInboxStore = createTypedStore(defineStore("inbox", () => {
    const storage = createLocalStorageStore<StorageState>({ key: "inbox.state", defaultValue: { limit: 10, offset: 0 } });
    const values = ref<Inbox[]>([]);

    const limit = ref(storage.load().limit);
    const offset = ref(storage.load().offset);

    async function loadInbox() {
        const req = await core.executeCommand('inbox-list', { limit: limit.value, offset: offset.value });
        if (req.success) {
            values.value = req.value;
        } else {
            console.error("Failed to load inbox:", req.error);
        }
    }

    async function createInboxItem(item: Partial<Inbox>) {
        const req = await core.executeCommand('inbox-create', item);
        if (req.success) {
            await loadInbox();
        } else {
            console.error("Failed to create inbox item:", req.error);
        }
    }

    async function updateInboxItem(item: Inbox) {
        const req = await core.executeCommand('inbox-update', item);
        if (req.success) {
            await loadInbox();
        } else {
            console.error("Failed to update inbox item:", req.error);
        }
    }

    async function deleteInboxItem(id: string) {
        const req = await core.executeCommand('inbox-delete', { id });
        if (req.success) {
            await loadInbox();
        } else {
            console.error("Failed to delete inbox item:", req.error);
        }
    }

    function setLimit(newLimit: number) {
        limit.value = newLimit;
        storage.save({ limit: limit.value, offset: offset.value });
        loadInbox();
    }

    function setOffset(newOffset: number) {
        offset.value = newOffset;
        storage.save({ limit: limit.value, offset: offset.value });
        loadInbox();
    }

    return {
        values,
        limit,
        offset,
        loadInbox,
        setLimit,
        setOffset,
        createInboxItem,
        updateInboxItem,
        deleteInboxItem,
    };
}));
