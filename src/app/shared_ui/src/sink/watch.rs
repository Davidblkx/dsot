use dioxus::prelude::*;
use tokio::sync::watch::Receiver;

pub fn use_receiver<T: Clone + 'static>(value: Receiver<T>) -> Signal<T> {
    let initial_value = value.borrow().clone();
    let mut signal = use_signal(|| initial_value);

    use_future(move || {
        let mut value = value.clone();

        async move {
            while value.changed().await.is_ok() {
                signal.set(value.borrow().clone());
            }
        }
    });

    signal
}
