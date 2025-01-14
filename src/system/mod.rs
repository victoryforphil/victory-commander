use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};
use victory_data_store::database::DataView;
use victory_data_store::topics::TopicKey;
use victory_time_rs::Timespan;

/// Trait that defines a commander system that can be run by the commander.
/// Will use its inputs to generate a data store query to feed said inputs into the system.
pub mod runner;

pub trait System {
    fn init(&mut self);
    fn get_subscribed_topics(&self) -> BTreeSet<TopicKey>;
    fn execute<'a>(&mut self, inputs: &'a DataView, dt: Timespan) -> DataView;
    fn cleanup(&mut self);
    fn name(&self) -> String {
        "".to_string()
    }
}

pub type SystemHandle = Arc<Mutex<dyn System>>;
