import type { StateTree, StoreDefinition } from "pinia";
import type { UnwrapRef } from "vue";

export type State<T> = {
    [K in keyof T]: UnwrapRef<T[K]>;
}

export function createTypedStore<TS extends StateTree, TC, TF>(def: StoreDefinition<string, TS, TC, TF>) : () => TF & State<TC> & State<TS> {
    return () => def();
}
