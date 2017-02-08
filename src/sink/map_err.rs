use sink::Sink;

use {Poll, StartSend};

/// Sink for the `Sink::map_err` combinator.
#[must_use = "sinks do nothing unless polled"]
pub struct MapErr<S, F> {
    sink: S,
    f: Option<F>,
}

pub fn new<S, F>(s: S, f: F) -> MapErr<S, F> {
    MapErr { sink: s, f: Some(f) }
}

impl<S, F, E> Sink for MapErr<S, F>
    where S: Sink,
          F: FnOnce(S::SinkError) -> E,
{
    type SinkItem = S::SinkItem;
    type SinkError = E;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.sink.start_send(item).map_err(|e| self.f.take().expect("cannot use MapErr after an error")(e))
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.sink.poll_complete().map_err(|e| self.f.take().expect("cannot use MapErr after an error")(e))
    }
}
