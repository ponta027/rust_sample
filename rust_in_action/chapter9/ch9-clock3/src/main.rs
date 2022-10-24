//
//
#[cfg(windows)]
use kernel32;
#[cfg(not(windows))]
use libc;
#[cfg(windows)]
use winapi;

use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, Duration as ChronoDuration, TimeZone, Timelike};
use chrono::{Local, Utc};
use clap::{App, Arg};
use std::mem::zeroed;
use std::net::UdpSocket;
use std::time::Duration;

const NTP_MESSAGE_LENGTH: usize = 48;
const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;
const LOCAL_ADDR: &'static str = "0.0.0.0:12300";

#[derive(Default, Debug, Copy, Clone)]
struct NTPTimestamp {
    seconds: u32,
    fraction: u32,
}

struct NTPMessage {
    data: [u8; NTP_MESSAGE_LENGTH],
}

#[derive(Debug)]
struct NTPResult {
    t1: DateTime<Utc>,
    t2: DateTime<Utc>,
    t3: DateTime<Utc>,
    t4: DateTime<Utc>,
}

impl NTPResult {
    fn offset(&self) -> i64 {
        let duration = (self.t2 - self.t1) + (self.t4 - self.t3);
        duration.num_milliseconds() / 2
    }

    fn delay(&self) -> i64 {
        let duration = (self.t4 - self.t1) + (self.t3 - self.t2);
        duration.num_milliseconds()
    }
}

/*
 *
 */
impl From<NTPTimestamp> for DateTime<Utc> {
    fn from(ntp: NTPTimestamp) -> Self {
        let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS;
        let mut nanos = ntp.fraction as f64;
        nanos *= 1e9;
        nanos /= 2_f64.powi(32);

        Utc.timestamp(secs, nanos as u32)
    }
}
/**
 *
 * */
impl From<DateTime<Utc>> for NTPTimestamp {
    fn from(utc: DateTime<Utc>) -> Self {
        let secs = utc.timestamp() + NTP_TO_UNIX_SECONDS;
        let mut fraction = utc.nanosecond() as f64;
        fraction *= 2_f64.powi(32);
        fraction /= 1e9;
        NTPTimestamp {
            seconds: secs as u32,
            fraction: fraction as u32,
        }
    }
}

///
///
impl NTPMessage {
    /// create instance
    fn new() -> Self {
        NTPMessage {
            data: [0; NTP_MESSAGE_LENGTH],
        }
    }
    /// create instance with VERSION,MODE
    fn client() -> Self {
        const VERSION: u8 = 0b00_011_000;
        const MODE: u8 = 0b00_000_011;
        let mut msg = NTPMessage::new();
        msg.data[0] |= VERSION;
        msg.data[0] |= MODE;
        msg
    }
    /// make slice  of first byte
    ///
    fn parse_timestamp(&self, i: usize) -> Result<NTPTimestamp, std::io::Error> {
        let mut reader = &self.data[i..i + 8];
        let seconds = reader.read_u32::<BigEndian>()?;
        let fraction = reader.read_u32::<BigEndian>()?;
        println!("parse_time:{:?},{:?}", seconds, fraction);
        Ok(NTPTimestamp {
            seconds: seconds,
            fraction: fraction,
        })
    }
    /// get rx time
    /// parse 32bit
    fn rx_time(&self) -> Result<NTPTimestamp, std::io::Error> {
        //
        self.parse_timestamp(32)
    }

    /// get tx time
    /// parse 40bit
    fn tx_time(&self) -> Result<NTPTimestamp, std::io::Error> {
        self.parse_timestamp(40)
    }
}

/// mean
fn weighted_mean(values: &[f64], weights: &[f64]) -> f64 {
    let mut result = 0.0;
    let mut sum_of_weights = 0.0;
    // v = value ,w =weight
    for (v, w) in values.iter().zip(weights) {
        result += v * w;
        sum_of_weights += w;
    }
    result / sum_of_weights
}

/// ntp
fn ntp_roundtrip(host: &str, port: u16) -> Result<NTPResult, std::io::Error> {
    //k
    let destination = format!("{}:{}", host, port);
    println!("destination:{}", destination);
    let timeout = Duration::from_secs(1);
    let request = NTPMessage::client();
    let mut response = NTPMessage::new();
    let message = request.data;

    println!("bind:{}", LOCAL_ADDR);
    let udp = UdpSocket::bind(LOCAL_ADDR)?;
    println!("bind");
    let result = udp.connect(&destination).expect("unable to connect");
    println!("connect:{:?}", result);
    let t1 = Utc::now();
    udp.send(&message);
    udp.set_read_timeout(Some(timeout))?;

    udp.recv_from(&mut response.data)?;
    println!("RECV {:?}", response.data);
    let t4 = Utc::now();
    let t2: DateTime<Utc> = response.rx_time().unwrap().into();
    let t3: DateTime<Utc> = response.tx_time().unwrap().into();
    Ok(NTPResult {
        t1: t1,
        t2: t2,
        t3: t3,
        t4: t4,
    })
}
///
fn check_time() -> Result<f64, std::io::Error> {
    //
    const NTP_PORT: u16 = 123;
    let servers = [
        "time.nist.gov",
        "time.apple.com",
        "time.euro.apple.com",
        "time.google.com",
        "time2.google.com",
        //"time.windows.com",
    ];
    let mut times = Vec::with_capacity(servers.len());
    for &server in servers.iter() {
        //
        print!("{} =>", server);
        let calc = ntp_roundtrip(&server, NTP_PORT);
        match calc {
            Ok(time) => {
                println!("{}ms away from local system time", time.offset());
                times.push(time);
            }
            Err(_) => {
                println!("? [response took too long]");
            }
        };
    }
    let mut offsets = Vec::with_capacity(servers.len());
    let mut offset_weights = Vec::with_capacity(servers.len());
    for time in &times {
        //
        let offset = time.offset() as f64;
        let delay = time.delay() as f64;
        let weight = 1_000_000.0 / (delay * delay);
        if weight.is_finite() {
            offsets.push(offset);
            offset_weights.push(weight);
        }
    }
    let avg_offset = weighted_mean(&offsets, &offset_weights);
    Ok(avg_offset)
}

struct Clock;
impl Clock {
    //
    ///
    fn get() -> DateTime<Local> {
        Local::now()
    }
    #[cfg(windows)]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {}
    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use libc::settimeofday;
        use libc::{suseconds_t, time_t, timeval, timezone};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ntp_roundtrip() {
        //
        let host = "localhost";
        let port = 80;
        let result: Result<NTPResult, std::io::Error> = ntp_roundtrip(&host, port);
        assert!(result.is_err());
    }

    /// なぜかcheck_timeをコールすると成功するが、ntp_roundtrippをコールすると
    /// エラーになる。
    #[test]
    fn test_ntp_roundtrip2() {
        //
        const NTP_PORT: u16 = 123;
        let servers = [
            "time.nist.gov",
            "time.apple.com",
            "time.euro.apple.com",
            "time.google.com",
            "time2.google.com",
            //"time.windows.com",
        ];
        let mut times = Vec::with_capacity(servers.len());
        for &server in servers.iter() {
            //
            print!("{} =>", server);
            let calc = ntp_roundtrip(&server, NTP_PORT);
            match calc {
                Ok(time) => {
                    println!("{}ms away from local system time", time.offset());
                    times.push(time);
                }
                Err(_) => {
                    println!("? [response took too long]");
                }
            };
        }
        assert!(false);
        /*
                let server = "time.nist.gov";
                const NTP_PORT: u16 = 123;
                let calc = ntp_roundtrip(&server, NTP_PORT);
                match &calc {
                    Ok(time) => {
                        println!("{}ms away from local system time", time.offset());
                    }
                    Err(_) => {
                        println!("? [response took too long]");
                    }
                };

                assert!(calc.is_ok());
        */
    }
    #[test]
    fn test_check_time() {
        //
        let result = check_time();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.0);
    }

    #[test]
    fn test_weighted_mean() {
        let values = [0.0, 0.0, 0.0];
        let weights = [0.0, 0.0, 0.0];
        let result = weighted_mean(&values, &weights);
        assert!(result.is_nan());
    }
    #[test]
    fn test_weighted_mean2() {
        let values = [0.0, 0.0, 0.0];
        let weights = [1.0, 0.0, 0.0];
        let result = weighted_mean(&values, &weights);
        assert_eq!(result, 0.0);
    }
    #[test]
    fn test_weighted_mean3() {
        let values = [1.0, 1.0, 0.0];
        let weights = [1.0, 1.0, 1.0];
        let result = weighted_mean(&values, &weights);
        assert_eq!(result, 2.0 / 3.0);
    }
    #[test]
    fn test_weighted_mean4() {
        let values = [2.0, 2.0, 1.0];
        let weights = [2.0, 1.0, 3.0];
        let result = weighted_mean(&values, &weights);
        assert_eq!(result, 9.0 / 6.0);
    }

    #[test]
    fn rx_time() {
        let mut instance = NTPMessage::client();
        instance.data[32] = 0b1010;
        let result = instance.rx_time().unwrap();
        assert_eq!(result.seconds, 0b0000_1010_0000_0000_0000_0000_0000_0000);
        assert_eq!(result.fraction, 0);
    }
    #[test]
    fn tx_time() {
        let mut instance = NTPMessage::client();
        instance.data[40] = 1;
        let result = instance.tx_time().unwrap();
        assert_eq!(result.seconds, 0b0000_0001_0000_0000_0000_0000_0000_0000);
        assert_eq!(result.fraction, 0);
    }
    #[test]
    fn NTP_Message_parse_time_stamp() {
        let instance = NTPMessage::client();
        //        instance.data[1] = 1;
        let result = instance.parse_timestamp(0).unwrap();
        assert_eq!(result.seconds, 0b11011000000000000000000000000);
        assert_eq!(result.fraction, 0);
    }
    #[test]
    fn new_NTPMessage_client() {
        let instance = NTPMessage::client();
        let mut data = [0; NTP_MESSAGE_LENGTH];

        data[0] = 0b00_011_011;

        assert_eq!(instance.data, data);
    }
    #[test]
    fn new_NTPMessage() {
        let instance = NTPMessage::new();
        let data = [0; NTP_MESSAGE_LENGTH];
        assert_eq!(instance.data, data);
    }
    #[test]
    fn utc_to_timestamp() {
        const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;
        let utc = Utc.timestamp(10, 0);
        let timestamp: NTPTimestamp = utc.into();
        assert_eq!(timestamp.seconds, (NTP_TO_UNIX_SECONDS as u32) + 10);
        assert_eq!(timestamp.fraction, 0);
        //
        //ut
    }
    #[test]
    fn timestamp() {
        const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;
        let timestamp = NTPTimestamp {
            seconds: (NTP_TO_UNIX_SECONDS as u32) + 1,
            fraction: 0,
        };
        let result: DateTime<Utc> = timestamp.into();
        assert_eq!(result.timestamp(), 1);
    }
    #[test]
    fn test_offset() {
        let t1 = Utc::now();
        let t2 = Utc::now();
        let t3 = Utc::now();
        let t4 = Utc::now();
        let result = NTPResult { t1, t2, t3, t4 };
        let offset = result.offset();
        assert_eq!(offset, 0);
    }
    #[test]
    fn test_offset_diff() {
        let t1 = Utc::now();
        let t2 = Utc::now();
        let t3 = Utc::now();
        let t4 = Utc::now() + ChronoDuration::milliseconds(1 as i64);
        let result = NTPResult { t1, t2, t3, t4 };
        let offset = result.offset();
        assert_eq!(offset, 0);
    }
    #[test]
    fn test_offset_diff_0() {
        let t1 = Utc::now();
        let t2 = Utc::now() + ChronoDuration::milliseconds(1 as i64);
        let t3 = Utc::now();
        let t4 = Utc::now();
        let result = NTPResult { t1, t2, t3, t4 };
        let offset = result.offset();
        assert_eq!(offset, 0);
    }
    #[test]
    fn test_delay() {
        let t1 = Utc::now();
        let t2 = Utc::now();
        let t3 = Utc::now();
        let t4 = Utc::now();
        let result = NTPResult { t1, t2, t3, t4 };
        let delay = result.delay();
        assert_eq!(delay, 0);
    }
}
fn main() {
    println!("Hello, world!");
    let action_arg = Arg::with_name("action")
        .takes_value(true)
        .possible_values(&["get", "set", "check-ntp"]);
    let use_standard_arg = Arg::with_name("std")
        .short("s")
        .long("use-standard")
        .takes_value(true)
        .possible_values(&["rfc2822", "rfc3339", "timestamp"])
        .default_value("rfc3339");
    let datetime_arg = Arg::with_name("dateime")
        .help("When <action> is 'set' appley <datetime>> . Otherwise , ignore.");
    let app = App::new("clock")
        .version("0.1.3")
        .about("Gets and sets the time")
        .after_help(" Note : UNIX timestampsare parsed")
        .arg(action_arg)
        .arg(use_standard_arg)
        .arg(datetime_arg);
    let args = app.get_matches();

    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();
    if action == "set" {
        let t_ = args.value_of("datetime").unwrap();
        let parser = match std {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),
        };
        let err_msg = format!("Unable to parse {}  according to {}", t_, std);
        let t = parser(t_).expect(&err_msg);
        Clock::set(t);
    } else if action == "check-ntp" {
        let offset = check_time().unwrap() as isize;
        let adjust_ms_ = offset.signum() * offset.abs().min(200) / 5;
        let adjust_ms = ChronoDuration::milliseconds(adjust_ms_ as i64);
        let now: DateTime<Utc> = Utc::now() + adjust_ms;
        Clock::set(now);
    }

    let maybe_error = std::io::Error::last_os_error();
    let os_error_code = &maybe_error.raw_os_error();
    match os_error_code {
        Some(0) => (),
        Some(_) => eprintln!("Unable to set the time:{:?}", maybe_error),
        None => (),
    }

    let now = Clock::get();

    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
