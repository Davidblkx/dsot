use dioxus::{core::provide_root_context, prelude::*};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PortalId(usize);

#[derive(Clone, Copy, PartialEq)]
struct PortalCtx {
    portals: Signal<HashMap<usize, Signal<Element>>>,
}

pub fn use_portals() -> () {
    let portals = Signal::new_in_scope(HashMap::new(), ScopeId::ROOT);
    let ctx = PortalCtx { portals };
    provide_root_context(ctx);
}

#[component]
pub fn Portal(children: Element) -> Element {
    static NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

    let (sig, id) = use_hook(|| {
        let mut next_id = NEXT_ID.write();
        let id = *next_id;
        *next_id += 1;

        let mut ctx = consume_context::<PortalCtx>();

        let sig = Signal::new_in_scope(children, ScopeId::ROOT);
        ctx.portals.write().insert(id, sig);

        (sig, PortalId(id))
    });

    use_drop(move || {
        let mut ctx = consume_context::<PortalCtx>();
        ctx.portals.write().remove(&id.0);
        sig.manually_drop();
    });

    rsx! {}
}

#[component]
pub fn PortalHost() -> Element {
    let ctx = use_context::<PortalCtx>();
    let portals_guard = ctx.portals.read();
    let child_elements: Vec<Signal<Element>> =
        portals_guard.iter().map(|(_, elem)| elem.clone()).collect();

    rsx! {
        for element in child_elements {
            {element}
        }
    }
}
