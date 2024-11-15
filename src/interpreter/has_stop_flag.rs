pub trait HasStopFlag {
    fn trigger_stop(&mut self);
    fn is_stopped(&self) -> bool;
}
