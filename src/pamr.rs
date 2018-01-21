fn dot(vec_a: &Vec<f64>, vec_b: &Vec<f64>) -> f64 {
    vec_a.iter().zip(vec_b.iter()).map(|(a, b)| a * b).sum()
}

pub fn pamr(
    insensitivity: f64,
    aggressiveness: f64,
    prices: &Vec<f64>,
    previous_prices: &Vec<f64>,
    allocation: &Vec<f64>,
) -> Vec<f64> {
    let returns: Vec<f64> = prices
        .iter()
        .zip(previous_prices.iter())
        .map(|(price, previous)| price / previous)
        .collect();

    let position = dot(allocation, &returns);

    let loss = f64::max(0.0, position - insensitivity);
    let mean: f64 = returns.iter().sum::<f64>() / returns.len() as f64;
    let returns_mean: Vec<f64> = returns.iter().map(|ret| ret - mean).collect();
    let returns_mean_squared: Vec<f64> = returns_mean.iter().map(|ret| ret.powi(2)).collect();
    let tau: Vec<f64> = returns_mean_squared
        .iter()
        .map(|ret| loss / (ret + (1. / (2. * aggressiveness))))
        .collect();
    let next_allocation: Vec<f64> = allocation
        .iter()
        .enumerate()
        .map(|(i, allocation)| (allocation - tau[i]) * returns_mean[i])
        .collect();

    let mut next_allocation_sorted: Vec<f64> = next_allocation.iter().map(|f| *f).collect();
    next_allocation_sorted.sort_unstable_by(|&v2, &v1| v1.partial_cmp(&v2).unwrap());

    let mut tmpsum = 0.;
    let mut tmax = 0.0;
    let mut baguete = false;
    for ii in 0..next_allocation_sorted.len() - 1 {
        tmpsum += next_allocation_sorted[ii];
        tmax = (tmpsum - 1.) / (ii as f64 + 1.);
        if tmax >= next_allocation_sorted[ii + 1] {
            baguete = true;
            break;
        }
    }
    if !baguete {
        let last = next_allocation_sorted[next_allocation_sorted.len() - 1];
        tmax = (tmpsum + last - 1.) / next_allocation_sorted.len() as f64;
    }

    let result: Vec<f64> = next_allocation
        .iter()
        .map(|a| f64::max(a - tmax, 0.))
        .collect();
    result
}
