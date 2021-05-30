use std::time::{SystemTime, UNIX_EPOCH};

// credits https://stackoverflow.com/a/44378174/2295672
pub fn get_unix_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    // println!("{:?}", since_the_epoch);
    let in_ms =
        since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    return in_ms;
}
