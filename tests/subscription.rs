pub mod common;

use common::msgs::example_msg::msg::Num;
use safe_drive::{context::Context, RecvResult};
use std::error::Error;

#[test]
fn test_subscription() -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    let ctx = Context::new()?;
    let node = ctx
        .create_node("test_subscription_node", None, Default::default())
        .unwrap();

    let subscription = node.create_subscriber::<Num>("test_subscription", Default::default())?;

    match subscription.try_recv() {
        RecvResult::RetryLater(_) => Ok(()), // must fail because there is no publisher
        _ => panic!(),
    }
}
