use std::collections::HashSet;



struct EventStream {
    recipients: HashSet<>
}

pub struct EventBus {

    event_stream: EventStream;

    pub fn dispatch() -> {}

    pub fn listen() -> {}
}
