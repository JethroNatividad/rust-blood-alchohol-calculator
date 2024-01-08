// make a program that calculates the blood alchohol content, and tell if it is legal to drive or not.
// Inputs: weight in lbs, gender, n of drinks, amount of alcohol by volume of the drinks consumed, hours since last drink.
// Process: get total oz of alchohol in oz, r = 0.73 for men else 0.66, bac = (A × 5.14 / W × r) − .015 × H
// Output: Your BAC is {}. /n It is not legal for you to drive.

enum Gender {
    Male = 0,
    Female = 1
}

fn round_decimal(number: f64, place: i32) -> f64 {
    let multiplier: f64 = 10_f64.powi(place);
    (number * multiplier).round() / multiplier
}

fn calculate_bac(weight: f64, gender: Gender , n_drink: f64, drink_alchohol_volume: f64, hours_since_last_drink: f64) -> f64 {
    let alchohol: f64 = n_drink * drink_alchohol_volume;
    let alchohol_distribution_ratio: f64 = match gender {
        Gender::Male => 0.73,
        Gender:: Female => 0.66,
    };

    round_decimal(((alchohol * 5.14) / (weight * alchohol_distribution_ratio)) - (0.015 * hours_since_last_drink), 2)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_bac() {
        assert_eq!(calculate_bac(130.0, Gender::Male, 5.0, 18.0, 2.0), 4.84);
        assert_eq!(calculate_bac(130.0, Gender::Female, 5.0, 18.0, 2.0), 5.36);
    }
}

fn main() {
    calculate_bac(130.0, Gender::Male, 5.0, 18.0, 2.0);
}
