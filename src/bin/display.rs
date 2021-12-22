use driver::{SupervisorEventForSingle::*, SupervisorForSingle};
use gnss::{LocalReference, WGS84};
use monitor_tool::{palette, rgba, vertex, Encoder};
use rtk_zhdgps::{ZhdH2, ZhdH2Msg};
use std::{net::UdpSocket, time::Duration};

const REF: WGS84 = WGS84 {
    latitude: 39.992516,
    longitude: 116.32737,
    altitude: 29.6446,
};

fn main() {
    let local = LocalReference::from(REF);
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let _ = socket.connect("127.0.0.1:12345");
    SupervisorForSingle::<ZhdH2>::default().join(|e| {
        match e {
            Connected(key, _) => println!("key = {}", key),
            Disconnected => println!("!"),
            ConnectFailed => {
                println!("?");
                std::thread::sleep(Duration::from_secs(1));
            }
            Event(_, Some((_, msg))) => match msg {
                ZhdH2Msg::BestPosa(data) => {
                    if data.state != "NONE" {
                        let enu = local.wgs84_to_enu(WGS84 {
                            latitude: data.latitude as f64,
                            longitude: data.longitude as f64,
                            altitude: data.altitude as f64,
                        });
                        let packet = Encoder::with(|figure| {
                            figure.with_topic("location", |mut topic| {
                                topic.set_color(0, rgba!(LIGHTBLUE; 1.0));
                                topic.push(vertex!(0; enu.e, enu.n; 0));
                            });
                        });
                        let _ = socket.send(&packet);
                    }
                    println!("{:?}", data);
                }
                ZhdH2Msg::Gpgga(_) => println!("gpgga"),
            },
            Event(_, None) => {}
        }
        true
    })
}
