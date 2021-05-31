use chrono::NaiveDateTime;

use cmd_crypto_chart::types::PriceInfo;

pub fn get_x_bounds(prices: &PriceInfo) -> [f64; 2] {
    // let mut min: u64 = u64::pow(2, 63) - 1;
    // let mut max: u64 = 0;
    let mut min = prices.first().unwrap().0;
    let mut max = prices.last().unwrap().0;

    let diff = max - min;
    min = min - diff / 20.0;
    max = max + diff / 20.0;
    // println!("{} {}", min, max);
    let ret: [f64; 2] = [min, max];

    return ret;
}

pub fn get_y_bounds(prices: &PriceInfo) -> [f64; 2] {
    let mut min: f64 = f64::powf(2.0, 62.0) - 1.0;
    let mut max: f64 = 0.0;

    for &e in prices {
        // println!("{}", e.1);
        if e.1 > max {
            max = e.1;
        }
        if e.1 < min {
            min = e.1;
        }
    }

    let diff = max - min;
    min = min - diff / 20.0;
    max = max + diff / 20.0;
    // println!("{} {}", min, max);
    let ret: [f64; 2] = [min, max];

    return ret;
}

pub fn get_x_labels(prices: &PriceInfo) -> Vec<String> {
    let bounds = get_x_bounds(prices);
    // we want to show 10 labels on y axis
    let step = (bounds[1] - bounds[0]) / 10.0;

    let mut labels: Vec<String> = Vec::new();
    let mut a = bounds[0];
    labels.push(get_display_time(a));

    for _ in 0..10 {
        a += step;
        labels.push(get_display_time(a));
    }

    // println!("{:?}", labels);
    return labels;
}

pub fn get_y_labels(prices: &PriceInfo) -> Vec<String> {
    let bounds = get_y_bounds(prices);
    // we want to show 10 labels on y axis
    let step = (bounds[1] - bounds[0]) / 10.0;

    let mut labels: Vec<String> = Vec::new();
    let mut a = bounds[0];
    labels.push(format!("{:.2}", a));
    // TODO: fails for coins which are less than 0.01 in price

    for _ in 0..10 {
        a += step;
        labels.push(format!("{:.2}", a));
    }

    // println!("{:?}", labels);
    return labels;
}

fn get_display_time(ts: f64) -> String {
    // convert milliseconds to seconds
    let ts_unix = (ts / 1000.0) as i64;
    let date_time = NaiveDateTime::from_timestamp(ts_unix, 0);
    // println!("Date was {}.", date_time);
    let time = date_time.time();
    // println!("Time was {}.", time);

    let time_string = time.format("%H:%M").to_string();
    // println!("Time formatted {}", timeString);

    return time_string;
}
