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
        setOffset
    };
}));
