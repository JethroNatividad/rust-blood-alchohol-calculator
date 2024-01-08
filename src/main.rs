// make a program that calculates the blood alchohol content, and tell if it is legal to drive or not.
// Inputs: weight in lbs, gender, n of drinks, amount of alcohol by volume of the drinks consumed, hours since last drink.
// Process: get total oz of alchohol in oz, r = 0.73 for men else 0.66, bac = (A × 5.14 / W × r) − .015 × H
// Output: Your BAC is {}. /n It is not legal for you to drive.

enum Gender {
    Male = 0,
    Female = 1
}

fn calculate_bac(weight: f64, gender: Gender, n_drink: f64, drink_volume: f64, hours_since_last_drink: f64) {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_bac() {
        assert_eq!(calculate_bac(130.0, 0, 5.0, 18.0, 2.0), 4.6)
    }
}

fn main() {
    println!("Hello, world!");
}
