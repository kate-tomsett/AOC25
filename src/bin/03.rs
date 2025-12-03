advent_of_code::solution!(3);

fn greedy_largest_subsequence(battery_bank: Vec<u64>, length_to_keep: usize) -> u64 {
    // This is the greedy largest subsequence algorithm
    // It takes a vector input and how many (k) digits we want to keep
    // It gives you the larges values of length k by removing some digits and doesn't change the relative order of the values.
    // Outputs a new k length vector
    // A chat-gpt special right here

    // This is our output vector of the largest k-long number we can make with our input.
    let mut output_joltage: Vec<u64> = Vec::new();
    let total_to_remove = battery_bank.len() - length_to_keep;// We want to retain k digits
    let mut removed = 0;

    // Loop over each original digit
    for &joltage in &battery_bank {
        // Get the last value in the new vector making sure it isnt empty
        while let Some(&last) = output_joltage.last() {
            // If our original digit is larger than what we have added to our new vector and we still have values to remove,
            // remove the smaller value in our new vector and replace with the original digit.
            if joltage > last && removed < total_to_remove {
                output_joltage.pop();
                removed += 1;
            } else {
                // If we cannot remove any more or if the value already in the vector is larger, keep it.
                break;
            }
        }
        output_joltage.push(joltage);
    }

    // If the values in the original vector were already in order, we would have an identical vector so truncate down to size.
    output_joltage.truncate(length_to_keep);

    // This creates a k digit u64 value out of our new vector of values by looping over the vector, mutiplying each previous value by 10 and adding our new vector element.
    output_joltage.iter().fold(0u64, |acc, &d| acc * 10 + d)
}

pub fn part_one(input: &str) -> Option<u64> {

    let mut total_joltage_output: u64 = 0;

    // Read in a line of joltages in a battery bank
    for battery_bank in input.split('\n') {

        // do not try and split the last empty line
        if battery_bank == ""{
            continue;
        }

        // Parse this list of joltages as a vector
        let battery_bank_digits: Vec<u64> = battery_bank
            .chars()                     // iterate over each character
            .filter_map(|c| c.to_digit(10)) // convert char to u32, skip non-digits
            .map(|d| d as u64)           // cast u32 → u64
            .collect();

        // Get the two largest values to get the 2 digit output joltage for this battery bank. Add on to the total.
        total_joltage_output += greedy_largest_subsequence(battery_bank_digits,2);
    }

    println!("Total joltage output: {total_joltage_output}");
    Some(total_joltage_output)
}

pub fn part_two(input: &str) -> Option<u64> {

    let mut total_joltage_output: u64 = 0;

    // Read in a line of joltages in a battery bank
    for battery_bank in input.split('\n') {

        // do not try and split the last empty line
        if battery_bank == ""{
            continue;
        }

        // Parse this list of joltages as a vector
        let battery_bank_digits: Vec<u64> = battery_bank
            .chars()                     // iterate over each character
            .filter_map(|c| c.to_digit(10)) // convert char to u32, skip non-digits
            .map(|d| d as u64)           // cast u32 → u64
            .collect();

        // Get the largest 12 digit output voltage we can from this battery bank. Add on to the total.
        // Apparently this is called the greedy largest subsequence algorithm which is cool.
        total_joltage_output += greedy_largest_subsequence(battery_bank_digits,12);

    }

    println!("Total joltage output: {total_joltage_output}");
    Some(total_joltage_output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
