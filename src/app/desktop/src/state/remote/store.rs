use dioxus::prelude::*;
use super::machine::RemoteMachine;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum SelectedMachine {
    Machine(usize),
    #[default]
    None,
}

#[derive(Debug, Clone, PartialEq, Default, Store)]
pub struct RemoteState {
    pub items: Vec<RemoteMachine>,
    pub selected: SelectedMachine,
}

pub type RemoteStore = Store<RemoteState>;
