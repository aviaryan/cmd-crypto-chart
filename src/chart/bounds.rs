// TODO: have a central type rather than duplicating here and in API
pub type PriceInfo = Vec<(u64, f32)>;

fn get_x_bounds_u64(prices: &PriceInfo) -> [u64; 2] {
	// let mut min: u64 = u64::pow(2, 63) - 1;
	// let mut max: u64 = 0;
	let mut min = prices.first().unwrap().0;
	let mut max = prices.last().unwrap().0;
	
	min = min - min / 20;
	max = max + max / 20;
	println!("{} {}", min, max);
	let ret: [u64; 2] = [min, max];

	return ret;
}

pub fn get_x_bounds(prices: &PriceInfo) -> [f64; 2] {
	let bounds = get_x_bounds_u64(prices);
	let ret: [f64; 2] = [bounds[0] as f64, bounds[1] as f64];
	// println!("hey {:?}", ret);
	return ret;
}

fn get_y_bounds_f32(prices: &PriceInfo) -> [f32; 2] {
	let mut min: f32 = f32::powf(2.0, 31.0) - 1.0;
	let mut max: f32 = 0.0;

	for &e in prices {
		// println!("{}", e.1);
		if e.1 > max {
			max = e.1;
		}
		if e.1 < min {
			min = e.1;
		}
	}
	
	min = min - min / 20.0;
	max = max + max / 20.0;
	println!("{} {}", min, max);
	let ret: [f32; 2] = [min, max];

	return ret;
}

pub fn get_y_bounds(prices: &PriceInfo) -> [f64; 2] {
	let bounds = get_y_bounds_f32(prices);
	let ret: [f64; 2] = [bounds[0] as f64, bounds[1] as f64];
	// println!("hey y bounds {:?}", ret);
	return ret;
}

pub fn get_x_labels(prices: &PriceInfo) -> Vec<String> {
	// let mut labels: Vec<u64> = Vec::new();
	// for (i, &ch) in prices.iter().enumerate() {
	// 	if i % 20 == 0 {
	// 		println!("{}: {}", i, ch.0);
	// 		labels.push(ch.0);
	// 	}
	// }
	// return labels;

	let bounds = get_x_bounds_u64(prices);
	// we want to show 10 labels on y axis
	let step = (bounds[1] - bounds[0]) / 10;

	let mut labels: Vec<String> = Vec::new();
	let mut a = bounds[0];
	labels.push(a.to_string());

	for x in 0..10 {
		a += step;
		labels.push(a.to_string());
	}

	println!("{:?}", labels);
	return labels;
}

pub fn get_y_labels(prices: &PriceInfo) -> Vec<String> {
	let bounds = get_y_bounds_f32(prices);
	// we want to show 10 labels on y axis
	let step = (bounds[1] - bounds[0]) / 10.0;

	let mut labels: Vec<String> = Vec::new();
	let mut a = bounds[0];
	labels.push(a.to_string());

	for x in 0..10 {
		a += step;
		labels.push(a.to_string());
	}

	println!("{:?}", labels);
	return labels;
}
