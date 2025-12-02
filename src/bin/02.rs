advent_of_code::solution!(2);

fn int_is_mirrored(n: &i64) -> bool {
    let s = n.to_string();
    let mid = s.len() /2;

    // split the id as a string at the mid point.
    let (first_half, second_half) = s.split_at(mid);

    // if the first half and the second half are the same, it is mirrored, so invalid.
    first_half == second_half
}

fn int_is_repeated(n: &i64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // try repeating different sections of the string. if the original number is made up of repeated parts, it is invalid.

    for size in 1..len {
        // check this attempted section size evenly divides the length of the string
        if len % size == 0{
            // get the first section of the original string in this section size
            let first_part = &s[..size];
            // repeat this section enough times to create a string the same length as the original
            let repeated_parts = first_part.repeat(len/size);

            // if this new string made up of repeated sections is the same as the original, it only contains repeated sections (2 or more) and is invalid.
            if repeated_parts == s{
                return true
            }
        }
    }

    // if the original string could not be recreated using repeated parts it must be valid.
    false
}

pub fn part_one(input: &str) -> Option<u64> {

    let mut total_invalid_ids: i64 = 0;

    // for each line in the input. In this case there should be one (plus the empty line)
    for line in input.split('\n') {

        // do not try and split the last empty line
        if line == ""{
            continue;
        }

        // Get each range of ids in the line by splitting on ','
        for range in line.split(','){
            // Get the first and last id in the range by splitting on '-'
            if let Some((first_id, last_id)) = range.split_once('-'){
                // Create a vector containing all the ids in that range, inclusive of the end id.
                let id_range: Vec<i64> = (first_id.parse::<i64>().unwrap()..=last_id.parse::<i64>().unwrap()).collect();
                for id in &id_range{
                    // if this id is mirrored it is invalid so add this to the total.
                    if int_is_mirrored(id){
                        total_invalid_ids += id;
                    }
                }
            }
        }
    }

    Some(total_invalid_ids.try_into().unwrap())
}


pub fn part_two(input: &str) -> Option<u64> {

    let mut total_invalid_ids: i64 = 0;

    // for each line in the input. In this case there should be one (plus the empty line)
    for line in input.split('\n') {

        // do not try and split the last empty line
        if line == ""{
            continue;
        }

        // Get each range of ids in the line by splitting on ','
        for range in line.split(','){
            // Get the first and last id in the range by splitting on '-'
            if let Some((first_id, last_id)) = range.split_once('-'){
                // Create a vector containing all the ids in that range, inclusive of the end id.
                let id_range: Vec<i64> = (first_id.parse::<i64>().unwrap()..=last_id.parse::<i64>().unwrap()).collect();
                for id in &id_range{
                    // if this id is made up of a repeated sequence, it is invalid so add to the total.
                    if int_is_repeated(id){
                        total_invalid_ids += id;
                    }
                }
            }
        }
    }

    // return the sum of the total invalid ids.
    Some(total_invalid_ids.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
