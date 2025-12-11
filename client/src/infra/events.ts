import type { Inbox } from "$pmodel/inbox.ts";
import { onMounted, onUnmounted } from "vue";
import { debugAction, debugLog } from "../device/settings.ts";

export type DsotEventMap = {
    "dsot:ready": never;
    "dsot:inbox:new": Partial<Inbox>;
};

export type DsotEventKey = keyof DsotEventMap;

export type DsotEvent<
    TKey extends DsotEventKey,
    TValue extends DsotEventMap[TKey],
> = CustomEvent<{
    key: TKey;
    value?: TValue;
    source?: Event;
}>;

export function triggerEvent<
    TKey extends DsotEventKey,
    TValue extends DsotEventMap[TKey],
>(
    key: TKey,
    value?: TValue,
    source?: Event,
): void {
    const event: DsotEvent<TKey, TValue> = new CustomEvent(key, {
        detail: {
            key,
            value,
            source,
        },
    });
    debugLog("Triggering DSOT event:", event);
    globalThis.dispatchEvent(event);
}

export function createDsotEventListener<
    TKey extends DsotEventKey,
    TValue extends DsotEventMap[TKey],
>(
    key: TKey,
    callback: (event: DsotEvent<TKey, TValue>) => void,
) {
    const actualCallback = (event: Event) => {
        const dsotEvent = event as DsotEvent<TKey, TValue>;
        debugLog("Handling DSOT event:", dsotEvent);

        callback(dsotEvent);
    };

    const startListening = debugAction(
        () => globalThis.addEventListener(key, actualCallback as EventListener),
        "Starting to listen for DSOT event:",
        key,
    );
    const stopListening = debugAction(
        () =>
            globalThis.removeEventListener(
                key,
                actualCallback as EventListener,
            ),
        "Stopping listening for DSOT event:",
        key,
    );

    return {
        startListening,
        stopListening,
    };
}

export function onDsotEvent<
    TKey extends DsotEventKey,
    TValue extends DsotEventMap[TKey],
>(
    key: TKey,
    callback: (event: DsotEvent<TKey, TValue>) => void,
) {
    const { startListening, stopListening } = createDsotEventListener(
        key,
        callback,
    );

    onMounted(() => {
        startListening();
    });

    onUnmounted(() => {
        stopListening();
    });
}
