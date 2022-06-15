use async_trait::async_trait;
use std::fmt::Debug;
use std::error::Error;

/// Describes a storing backend that can 
/// save and load given internal type of updates for 
/// state.
#[async_trait]
pub trait StateBackend {
    /// Aggregated state in memory
    type State: Clone + State + 'static;
    /// Errors that can occur in the backend
    type Err: Clone + Debug + Error + 'static;

    /// Write down state update into storage
    async fn write(&mut self, upd: SnapshotedUpdate<Self::State>) -> Result<(), Self::Err>;

    /// Collect all updates until first snapshot in the chain
    async fn updates(&self) -> Result<Vec<SnapshotedUpdate<Self::State>>, Self::Err>;
}

/// Aggregated state that could be updated by small updates
pub trait State {
    /// Incremental single update of the state
    type Update: Clone + PartialEq + Send + 'static; 
    /// Update error
    type Err: Clone + Debug + Error + 'static;

    /// Update the state with incremental part
    fn update(&mut self, upd: Self::Update) -> Result<(), Self::Err>;
}

/// Update with added shapshot to capture points when 
/// we want to save whole state.
#[derive(Debug, Clone, PartialEq)]
pub enum SnapshotedUpdate<St: State> {
    Incremental(St::Update),
    Snapshot(St),
}

impl<St: State> SnapshotedUpdate<St> {
    /// True if update is snapshot
    pub fn is_snapshot(&self) -> bool {
        match self {
            SnapshotedUpdate::Snapshot(_) => true,
            _ => false,
        }
    }
}