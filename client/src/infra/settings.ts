function loadStorageItem<T>(storage: Storage, key: string, defaultValue: T): T {
    const storedValue = storage.getItem(key);
    if (storedValue) {
        try {
            const value = {
                ...defaultValue,
                ...JSON.parse(storedValue),
            } as T;

            saveStorageItem(storage, key, value, true);

            return value;
        } catch {
            return defaultValue;
        }
    } else {
        saveStorageItem(storage, key, defaultValue, true);
    }
    return defaultValue;
}

let saveId: number | undefined;
function saveStorageItem<T>(storage: Storage, key: string, value: T, skipDebounce?: boolean): void {
    if (skipDebounce) {
        storage.setItem(key, JSON.stringify(value));
        return;
    }

    clearTimeout(saveId);

    saveId = setTimeout(() => {
        storage.setItem(key, JSON.stringify(value));
    }, 5000); // Debounce for 5 seconds
}

export class Settings<T> {
    #key: string;
    #storage: Storage;
    #defaultValue: T;
    #value: T;

    constructor({
        key,
        defaultValue,
        storage = localStorage,
    }: {
        key: string;
        defaultValue: T;
        storage?: Storage;
    }) {
        this.#key = key;
        this.#storage = storage;
        this.#defaultValue = defaultValue;
        this.#value = loadStorageItem(this.#storage, this.#key, this.#defaultValue);
    }

    public get<K extends keyof T>(key: K): T[K] {
        const storedValue = this.#storage.getItem(this.#key);
        if (storedValue) {
            try {
                const parsed = JSON.parse(storedValue) as T;
                return parsed[key];
            } catch {
                return this.#defaultValue[key];
            }
        }
        return this.#defaultValue[key];
    }

    public set<K extends keyof T>(key: K, value: T[K]): void {
        this.#value[key] = value;
        saveStorageItem(this.#storage, this.#key, this.#value);
    }
}
