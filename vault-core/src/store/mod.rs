pub mod event;
pub mod mutation_event;
pub mod mutation_state;
pub mod state;
pub mod test_helpers;

use std::sync::Arc;

pub use vault_store::{update_if, NextId};

pub use self::{
    event::Event, mutation_event::MutationEvent, mutation_state::MutationState, state::State,
};

pub type EventEmitter = vault_store::EventEmitter<Event, MutationState>;

pub type MutationEventEmitter =
    vault_store::MutationEventEmitter<State, Event, MutationState, MutationEvent>;

pub type Notify = vault_store::Notify<Event>;

pub type MutationNotify = vault_store::MutationNotify<MutationEvent, State, MutationState>;

pub type Store = vault_store::Store<State, Event, MutationState, MutationEvent>;

pub type Subscription = vault_store::Subscription<State, Event, MutationState, MutationEvent>;

pub async fn wait_for<F: Fn() -> Option<R> + Send + Sync + 'static, R: Send + 'static>(
    store: Arc<Store>,
    events: &[Event],
    f: F,
) -> R {
    vault_store::wait_for(store, events, f).await
}
