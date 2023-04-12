use async_graphql::{Enum, SimpleObject};
use std::collections::HashMap;

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

#[derive(Clone, Default)]
pub struct SubscriptionManager {
    pub subscriptions: HashMap<String, String>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_subscription(&mut self, _name: String) {
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
