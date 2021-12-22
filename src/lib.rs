use driver::Driver;
use serial_port::{Port, PortKey, SerialPort};
use std::{
    sync::{Arc, Weak},
    time::{Duration, Instant},
};

const OPEN_TIMEOUT: Duration = Duration::from_millis(3000);
const LINE_RECEIVE_TIMEOUT: Duration = Duration::from_millis(2500);

mod bestposa;
mod buffer;

use buffer::Buffer;

pub struct ZhdH2 {
    port: Arc<Port>,
    buf: Buffer<512>,
    last_time: Instant,
}

pub enum ZhdH2Msg {
    BestPosa(u16, f32, String),
    Gpgga(String),
}

pub struct RTCMReceiver(Weak<Port>);

impl RTCMReceiver {
    pub fn receive(&self, buf: &[u8]) {
        if let Some(p) = self.0.upgrade() {
            let _ = p.write(buf);
        }
    }
}

impl ZhdH2 {
    pub fn get_receiver(&self) -> RTCMReceiver {
        RTCMReceiver(Arc::downgrade(&self.port))
    }
}

impl Driver for ZhdH2 {
    type Pacemaker = ();
    type Key = PortKey;
    type Event = ZhdH2Msg;

    fn keys() -> Vec<Self::Key> {
        Port::list().into_iter().map(|id| id.key).collect()
    }

    fn open_timeout() -> std::time::Duration {
        OPEN_TIMEOUT
    }

    fn new(t: &Self::Key) -> Option<(Self::Pacemaker, Self)> {
        Port::open(t, 115200, LINE_RECEIVE_TIMEOUT.as_millis() as u32)
            .ok()
            .map(|port| {
                (
                    (),
                    Self {
                        port: Arc::new(port),
                        buf: Buffer::new(),
                        last_time: Instant::now(),
                    },
                )
            })
    }

    fn join<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(&mut Self, Option<(std::time::Instant, Self::Event)>) -> bool,
    {
        let mut time = Instant::now();
        loop {
            if let Some(line) = self.buf.parse() {
                time = self.last_time;
                let event = if let Some(msg) = bestposa::split(line) {
                    Some((time, msg))
                } else if line.starts_with("$GPGGA,") {
                    Some((time, ZhdH2Msg::Gpgga(format!("{}\r\n", line))))
                } else {
                    None
                };
                if !f(self, event) {
                    return true;
                }
            } else if self.last_time > time + LINE_RECEIVE_TIMEOUT {
                return false;
            } else {
                let buf = self.buf.to_write();
                if let Some(n) = self.port.read(buf).filter(|n| *n > 0) {
                    self.last_time = Instant::now();
                    self.buf.extend(n);
                }
                // 接收失败
                else {
                    return false;
                }
            }
        }
    }
}
