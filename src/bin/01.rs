
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {

    // how many times the dial hits 0
    let mut landed_zero_count = 0;
    // dial starts at 50
    let mut dial_count = 50;

    for line in input.split('\n') {
        // do not try and split the last empty line
        if line == ""{
            continue;
        }

        // get whether we are going left or right, and by how many
        let direction = &line[0..1];
        let mut delta = line[1..].parse::<i32>().unwrap();

        // if we are going left, treat the current count as negative
        if direction == "L"{
            delta *= -1;
        }

        // get the non-negative remainder of adding this new count to the total
        dial_count = (dial_count + delta).rem_euclid(100);

        // if this count is zero, the dial has landed there so add one to our zero dial count
        if dial_count == 0 {
            landed_zero_count += 1;
        }
    }

    println!("Number of zeros: {landed_zero_count}");
    // return the number of times the dial landed on zero
    Some(landed_zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {

    // how many times the dial PASSES zero
    let mut passed_zero_count = 0;
    // dial starts on zero
    let mut dial_count = 50;

    for line in input.split('\n') {
        // do not try and split last empty line.
        if line == ""{
            continue;
        }

        // get the direction, left or right, and how much we are moving the dial by
        let direction = &line[0..1];
        let mut delta = line[1..].parse::<i32>().unwrap();

        // get the number of times this delta divides by 100 as we will pass zero at least this many times
        // also get the remainder when divided by 100 as this will affect our ending dial position
        let res = (delta / 100, delta % 100);
        passed_zero_count += res.0;

        if direction == "L"{
            // if we are going left, and our end dial count will be less than zero, we will have passed zero.
            // do not change the zero count if the total count is zero, as we have already dealt with that
            if dial_count != 0 && dial_count - res.1 <= 0{
                passed_zero_count += 1;
            }
            // now treat this delta as negative to calculate final non-negative dial position
            delta *= -1;
        }
        else {
            // if we are going right, and our end dial count will be more than 100, we will pass zero.
            if dial_count + res.1 >= 100{
                passed_zero_count += 1;
            }
        }

        // get the non-negative remainder of adding this new count to the total
        dial_count = (dial_count + delta).rem_euclid(100);
    }

    println!("Number of zeros: {passed_zero_count}");
    // return the number of times the dial passed zero
    Some(passed_zero_count.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
