#[derive(Debug)]
pub struct BestPosa {
    pub weeks: u16,
    pub seconds: f32,
    pub state: String,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
    pub error_altitude: f32,
    pub sigma_latitude: f32,
    pub sigma_longitude: f32,
    pub sigma_altitude: f32,
    pub diff_id: String,
    pub diff_age: f32,
    pub delay: f32,
    pub satellite: (u8, u8, u8, u8),
}

impl BestPosa {
    pub fn from(line: &str) -> Option<Self> {
        if !line.starts_with("#BESTPOSA,") {
            return None;
        }
        let (head, body) = line.split_once(';')?;
        let mut head = head.split(',').skip(5);
        let mut body = body.split(',').skip(1);
        let data = BestPosa {
            weeks: head.next()?.parse().ok()?,
            seconds: head.next()?.parse().ok()?,
            state: body.next()?.into(),
            latitude: body.next()?.parse().ok()?,
            longitude: body.next()?.parse().ok()?,
            altitude: body.next()?.parse().ok()?,
            error_altitude: body.next()?.parse().ok()?,
            sigma_latitude: {
                body.next()?;
                body.next()?.parse().ok()?
            },
            sigma_longitude: body.next()?.parse().ok()?,
            sigma_altitude: body.next()?.parse().ok()?,
            diff_id: body.next()?.trim_matches('"').into(),
            diff_age: body.next()?.parse().ok()?,
            delay: body.next()?.parse().ok()?,
            satellite: (
                body.next()?.parse().ok()?,
                body.next()?.parse().ok()?,
                body.next()?.parse().ok()?,
                body.next()?.parse().ok()?,
            ),
        };
        Some(data)
    }
}
