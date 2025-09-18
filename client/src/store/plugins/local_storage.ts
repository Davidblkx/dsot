export interface LocalStorageStore<T> {
    load(): T;
    save(data: T): void;
}

/**
 * Logic to load and save data to localStorage
 *
 * @param options
 * @param options.key The key to use in localStorage
 * @param options.defaultValue The default value to return if no data is found
 * @param options.storage The storage object to use (defaults to localStorage)
 * @returns
 */
export function createLocalStorageStore<T>({
    key,
    defaultValue,
    storage = localStorage
}: {
    key: string;
    defaultValue: T;
    storage?: Storage;
}): LocalStorageStore<T> {
    const store_key = `dsot_${key}`;
    return {
        load(): T {
            const data = storage.getItem(store_key);
            if (data) {
                try {
                    return JSON.parse(data) as T;
                } catch (err) {
                    console.error(`Error parsing localStorage key "${key}":`, err);
                    return defaultValue;
                }
            }
            return defaultValue;
        },
        save(data: T): void {
            storage.setItem(store_key, JSON.stringify(data));
        }
    };
}
