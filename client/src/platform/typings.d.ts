export type Result<T> = {
    success: true;
    value: T;
} | {
    success: false;
    error: string;
};

export interface Platform {
    getRuntimeName(): string;
}

export const core: Platform;
