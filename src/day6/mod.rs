// For each idx, calculate how many ways we can beat the distance in the given time.
// Total distance = Speed * (Total time - speed) (note this is also a mirrored function)

pub fn solve1(input: Vec<String>) -> i32 {
    let times: Vec<i32> = input.iter().nth(0).unwrap().split(':').nth(1).unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let distances: Vec<i32> = input.iter().nth(1).unwrap().split(':').nth(1).unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut ret: i32 = 1;


    for i in 0..times.len() {
        let mut total_ways: i32 = 0;
        let race_total_time = times[i];
        let race_distance = distances[i];

        for x in 0..race_total_time as i32 {
            if x * (race_total_time - x) > race_distance {
                total_ways += 1
            }
        }
        ret *= total_ways;
    }
    ret
}

// Instead of brute-forcing, we should instead consider this function as an inverse parabola, find the zeroes, and consider the domain where y is positive.
// If Total distance = Speed * (Total time - speed) (note this is also a mirrored function)
// Let total_distance = y, speed = x, Total_time = T (constant), Race_distance = R, then we need to sovle
// y = -x^2 + Tx when y > R, or -x^2 + Tx > R
// Which resolves to: -x^2 + Tx -R > 0

pub fn solve2(input: Vec<String>) -> i128 {
    let time: i128 = input.iter().nth(0).unwrap().split(':').nth(1).unwrap().replace(' ', "").parse::<i128>().unwrap();
    let distance: i128 = input.iter().nth(1).unwrap().split(':').nth(1).unwrap().replace(' ', "").parse::<i128>().unwrap();

    let a: f64 = -1.0;
    let b: f64 = time as f64; // T
    let c: f64 = - distance as f64; // R

    // Compute the zeros of the function using quadratic formula (-b +- sqrt(b^2 - 4ac)/2a )
    let x1 = (- b as f64 + (((b * b) - 4.0 * a * c) as f64).sqrt()) / (2.0 * a);
    let x2 = (- b as f64 - (((b * b) - 4.0 * a * c) as f64).sqrt()) / (2.0 * a);

    // Get the distance of the range between the zeroes (we dont care about the order, we just want the magnitude of the range). Take floor due to any float inaccuracies
    (x1 - x2).abs().floor() as i128
}









