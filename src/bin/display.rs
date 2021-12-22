use driver::{SupervisorEventForSingle::*, SupervisorForSingle};
use rtk_zhdgps::{ZhdH2, ZhdH2Msg};
use std::time::Duration;

fn main() {
    SupervisorForSingle::<ZhdH2>::default().join(|e| {
        match e {
            Connected(key, _) => println!("key = {}", key),
            Disconnected => println!("!"),
            ConnectFailed => {
                println!("?");
                std::thread::sleep(Duration::from_secs(1));
            }
            Event(_, Some((_, msg))) => match msg {
                ZhdH2Msg::BestPosa(weeks, seconds, body) => {
                    println!("{:#}:{:#}:{}", weeks, seconds, body)
                }
                ZhdH2Msg::Gpgga(_) => println!("gpgga"),
            },
            Event(_, None) => {}
        }
        true
    })
}
