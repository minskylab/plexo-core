use std::time::Duration;

use async_graphql::{futures_util::StreamExt, Subscription};
use tokio_stream::Stream;

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| {
                value += step;
                value
            })
    }
    // async fn tasks(&self) -> impl Stream<Item = Task> {
    //     tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
    //         .map(|_| Task {
    //             id: "1".to_string(),
    //             title: "Task 1".to_string(),
    //             created_at: None,
    //             updated_at: None,
    //             description: None,
    //         })
    // }
}
