import { ref as vue_ref, type Ref } from "vue";

export function ref<T>(value: T): Ref<T> {
    return vue_ref(value) as unknown as Ref<T>;
}
