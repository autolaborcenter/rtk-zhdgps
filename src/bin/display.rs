use async_std::{net::UdpSocket, sync::Arc, task};
use driver::{SupervisorEventForSingle::*, SupervisorForSingle};
use gnss::{LocalReference, WGS84};
use monitor_tool::{palette, rgba, vertex, Encoder};
use rtk_zhdgps::{ZhdH2, ZhdH2Msg};
use std::time::Duration;

const REF: WGS84 = WGS84 {
    latitude: 39.992516,
    longitude: 116.32737,
    altitude: 29.6446,
};

fn main() {
    let local = LocalReference::from(REF);
    task::block_on(async move {
        let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await.unwrap());
        let _ = socket.connect("127.0.0.1:12345").await;
        send_config(socket.clone(), Duration::from_secs(5));

        SupervisorForSingle::<ZhdH2>::default().join(|e| {
            match e {
                Connected(key, _) => println!("key = {}", key),
                Disconnected => println!("!"),
                ConnectFailed => {
                    println!("?");
                    task::block_on(task::sleep(Duration::from_secs(1)));
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
                                    topic.push(vertex!(0; enu.e, enu.n; 0))
                                });
                            });
                            let _ = task::block_on(socket.send(&packet));
                        }
                        println!("{:?}", data);
                    }
                    ZhdH2Msg::Gpgga(_) => println!("gpgga"),
                },
                Event(_, None) => {}
            }
            true
        });
    });
}

fn send_config(socket: Arc<UdpSocket>, period: Duration) {
    let packet = Encoder::with(|figure| {
        figure.config_topic("location", 20000, 0, &[(0, rgba!(LIGHTBLUE; 0.5))], |_| {});
    });
    task::spawn(async move {
        let clear = Encoder::with(|encoder| {
            encoder.topic("location").clear();
        });
        let _ = socket.send(clear.as_slice()).await;
        loop {
            let _ = socket.send(&packet).await;
            task::sleep(period).await;
        }
    });
}
