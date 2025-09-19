import { defineComponent } from 'vue';

export default defineComponent({
    name: 'Section',
    setup(_props, { slots }) {
        return () => (
            <section data-component="section">
                {slots.title ? <header>
                    {slots.title?.()}
                </header> : null}
                <div>
                    {slots.default?.()}
                </div>
                {
                    slots.footer ? <footer>
                        {slots.footer?.()}
                    </footer> : null
                }
            </section>
        )
    }
})
