import { defineComponent, Teleport } from 'vue';

export default defineComponent({
  name: 'Window',
  setup(_props, { slots }) {
    return () => (
        <Teleport to="body">
            <div data-component="window">
                {slots.default?.()}
            </div>
        </Teleport>
    );
  },
});
