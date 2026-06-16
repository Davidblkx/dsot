use dioxus::{core::provide_root_context, prelude::*};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PortalId(usize);

#[derive(Clone, Copy, PartialEq)]
struct PortalCtx {
    portals: Signal<HashMap<String, HashMap<usize, Signal<Element>>>>,
}

pub fn use_portals() -> () {
    let portals = Signal::new_in_scope(HashMap::new(), ScopeId::ROOT);
    let ctx = PortalCtx { portals };
    provide_root_context(ctx);
}

#[derive(Props, Clone, PartialEq)]
pub struct PortalProps {
    children: Element,
    #[props(into, default)]
    host: String,
}

#[component]
pub fn Portal(props: PortalProps) -> Element {
    static NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

    let host_id = props.host.clone();
    let host_id_for_hook = host_id.clone();
    let (sig, id) = use_hook(move || {
        let mut next_id = NEXT_ID.write();
        let id = *next_id;
        *next_id += 1;

        let mut ctx = consume_context::<PortalCtx>();

        let sig = Signal::new_in_scope(props.children, ScopeId::ROOT);
        
        ctx.portals.write()
            .entry(host_id_for_hook)
            .or_default()
            .insert(id, sig);

        (sig, PortalId(id))
    });

    use_drop(move || {
        let mut ctx = consume_context::<PortalCtx>();
        let mut portals = ctx.portals.write();
        if let Some(host_portals) = portals.get_mut(&host_id) {
            host_portals.remove(&id.0);
            if host_portals.is_empty() {
                portals.remove(&host_id);
            }
        }
        sig.manually_drop();
    });

    rsx! {}
}

#[derive(Props, Clone, PartialEq)]
pub struct PortalHostProps {
    #[props(into, default)]
    id: String,
}

#[component]
pub fn PortalHost(props: PortalHostProps) -> Element {
    let ctx = use_context::<PortalCtx>();
    let portals_guard = ctx.portals.read();
    let child_elements: Vec<Signal<Element>> = if let Some(host_portals) = portals_guard.get(&props.id) {
        let mut elements: Vec<(usize, Signal<Element>)> = host_portals.iter().map(|(id, elem)| (*id, elem.clone())).collect();
        elements.sort_by_key(|(id, _)| *id);
        elements.into_iter().map(|(_, elem)| elem).collect()
    } else {
        Vec::new()
    };

    rsx! {
        for element in child_elements {
            {element}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    thread_local! {
        static PORTALS: std::cell::RefCell<Option<Signal<HashMap<String, HashMap<usize, Signal<Element>>>>>> = const { std::cell::RefCell::new(None) };
    }

    #[test]
    fn test_portals_with_multiple_hosts() {
        fn app() -> Element {
            use_portals();
            
            let ctx = consume_context::<PortalCtx>();
            PORTALS.with(|p| *p.borrow_mut() = Some(ctx.portals));

            rsx! {
                PortalHost { id: "host-a" }
                PortalHost { id: "host-b" }
                PortalHost {}

                Portal {
                    host: "host-a",
                    div { id: "a1", "A1" }
                }
                Portal {
                    host: "host-b",
                    div { id: "b1", "B1" }
                }
                Portal {
                    div { id: "d1", "Default" }
                }
            }
        }

        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();

        let portals = PORTALS.with(|p| p.borrow_mut().take().expect("PortalCtx portals signal not found"));
        let portals_guard = portals.read();

        assert!(portals_guard.contains_key("host-a"));
        assert_eq!(portals_guard.get("host-a").unwrap().len(), 1);

        assert!(portals_guard.contains_key("host-b"));
        assert_eq!(portals_guard.get("host-b").unwrap().len(), 1);

        assert!(portals_guard.contains_key(""));
        assert_eq!(portals_guard.get("").unwrap().len(), 1);
    }
}
