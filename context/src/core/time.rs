#[derive(Debug)]
pub struct Time {
    initial_time: f64,
}

impl Time {
    pub fn new() -> Time {
        Time {
            initial_time: Time::convert(time::get_time()),
        }
    }

    fn convert(time_stamp: time::Timespec) -> f64 {
        let n_sec_part = time_stamp.nsec as f64 * 0.000_000_001;
        let sec_part = time_stamp.sec as f64;

        sec_part + n_sec_part
    }

    pub fn time(&self) -> f64 {
        Time::convert(time::get_time()) - self.initial_time
    }
}
