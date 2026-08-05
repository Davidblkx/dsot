use dioxus::prelude::*;
use dsot_lib::{
    DsotCore,
    user::{LocalUserCredentials, LocalUserPathProvider},
};

use crate::components::UserSelect;

#[component]
pub fn UserView(children: Element) -> Element {
    let core = use_context::<DsotCore>();
    let initial_is_login = *core.user.is_logged_in().borrow();
    let user = core.user.clone();

    let mut is_login = use_signal(|| initial_is_login);

    use_future(move || {
        let mut watch = user.is_logged_in().clone();

        async move {
            while watch.changed().await.is_ok() {
                is_login.set(*watch.borrow());
            }
        }
    });

    let select_user = move |id: String| {
        let user_path = core.get_user_path(id);
        let user = core.user.clone();

        async move {
            // Should this be in this view, maybe call it LocalUserView?
            match user.login_local(user_path, LocalUserCredentials::SkipValidation) {
                Ok(_) => log::info!("User logged in successfully"),
                Err(e) => log::error!("{}", e),
            }
        }
    };

    rsx! {
        if *is_login.read() {
            {children}
        } else {
            div {
                "data-view": "user",
                UserSelect {
                    is_empty: rsx! {
                        div {
                            class: "empty",
                            "Create a new user"
                        }
                    },
                    on_user_select: move |user_id| {
                        let waiter = select_user(user_id);

                        spawn(async move {
                            waiter.await;
                        });
                    }
                }
            }
        }
    }
}
