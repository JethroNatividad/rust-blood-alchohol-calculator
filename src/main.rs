// make a program that calculates the blood alchohol content, and tell if it is legal to drive or not.
// Inputs: weight in lbs, gender, n of drinks, amount of alcohol by volume of the drinks consumed, hours since last drink.
// Process: get total oz of alchohol in oz, r = 0.73 for men else 0.66, bac = (A × 5.14 / W × r) − .015 × H
// Output: Your BAC is {}. /n It is not legal for you to drive.

enum Gender {
    Male: u32 = 0,
    Female: u32 = 1
}

fn round_decimal(number: f64, place: i32) -> f64 {
    let multiplier: f64 = 10_f64.powi(place);
    (number * multiplier).round() / multiplier
}

fn calculate_bac(weight: f64, gender: Gender , n_drinks: f64, drink_alchohol_volume: f64, hours_since_last_drink: f64) -> f64 {
    let alchohol: f64 = n_drinks * drink_alchohol_volume;
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

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}


def get_gender() -> Gender {
    loop {
        let gender_input: u32 = get_input("What is your gender? (0 for Male, 1 for Female) ");

        match gender_input {
            0 => break Gender::Male,
            1 => break Gender::Female,
            _ => println!("Invalid input. Please try again.");
        }
    }
}

fn main() {
    let weight_lbs: f64 = get_input("What is your weight in lbs? ");
    let gender: Gender = get_gender();
    let n_drinks: f64 = get_input("How many drinks did you drink? ");
    let drink_alchohol_volume: f64 = get_input("How much alchohol(oz) was in per drink? ");
    let hours_since_last_drink: f64 = get_input("How many hours since your last drink? ");

}
