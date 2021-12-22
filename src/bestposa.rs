use crate::ZhdH2Msg;

pub fn split(line: &str) -> Option<ZhdH2Msg> {
    if !line.starts_with("#BESTPOSA,") {
        return None;
    }
    let (head, tail) = line.split_once(';')?;
    let mut time = head.split(',').skip(5);
    let weeks = time.next()?.parse::<u16>().ok()?;
    let seconds = time.next()?.parse::<f32>().ok()?;
    Some(ZhdH2Msg::BestPosa(weeks, seconds, tail.into()))
}
