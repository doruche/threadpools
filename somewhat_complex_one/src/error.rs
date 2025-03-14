#![allow(unused)]

#[derive(Debug, PartialEq)]
pub enum Error {
    Empty,
    Cancelled,
    Timeout,
    MultipleWaits,
    ChannelDisconnected,
    CancelAfterRunning,
    Other(String),
}