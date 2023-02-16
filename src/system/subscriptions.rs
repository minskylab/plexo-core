use async_graphql::{Enum, SimpleObject};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::sync::Mutex;
use tokio::time::{interval, Duration, Instant};
use tokio_stream::wrappers::IntervalStream;

use tokio_stream::StreamExt;
use uuid::Uuid;

// use tokio_stream::StreamExt;
// use uuid::Uuid;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum DataDiffEventKind {
    Created,
    Updated,
    Deleted,
}

#[derive(SimpleObject, Clone)]
pub struct DataDiffEvent {
    pub kind: DataDiffEventKind,
    pub data: String,
}

#[derive(Clone)]
pub struct SubscriptionManager {
    // pub tasks_subscription: Arc<IntervalStream>,
    pub subscriptions: HashMap<String, Arc<IntervalStream>>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
        }
    }

    pub fn add_subscription(&mut self, name: String) {
        let stream_name = "table:tasks".to_string();

        let interval_stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(1)));

        let mapped_stream = interval_stream.map(move |_| DataDiffEvent {
            kind: DataDiffEventKind::Created,
            data: Uuid::new_v4().to_string(),
        });

        let stream = Arc::new(mapped_stream);

        // self.subscriptions.insert(stream_name, stream);

        // let (tx, rx) = unbounded_channel();
        // let interval = IntervalStream::new(interval(Duration::from_secs(1)));
        // let data = "some data".to_owned();

        // tokio::spawn(async move {
        //     loop {
        //         interval.next().await;
        //         tx.send(DataDiffEvent {
        //             kind: DataDiffEventKind::Created,
        //             data: data.clone(),
        //         })
        //         .unwrap();
        //     }
        // });

        // self.subscriptions.insert(name, rx);
    }
}
