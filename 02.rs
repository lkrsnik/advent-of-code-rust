advent_of_code::solution!(2);

fn read(complete_input: &str) -> Vec<Vec<i32>> {
    let rows: Vec<_> = complete_input.split("\n").collect();
    let mut vect: Vec<Vec<i32>> = Vec::new();

    for row in rows {
        let elems: Vec<_> = row.split(" ").collect();
        let mut vec: Vec<i32> = Vec::new();
        for el in elems {
            if el != "" {
                let my_i32 = el.parse::<i32>().unwrap();
                vec.push(my_i32);
            }
        }
        vect.push(vec)
    }

    vect
}

pub fn part_one(input: &str) -> Option<u32> {

    let vector_input: Vec<Vec<i32>> = read(input);
    let mut count_success: u32 = 0;
    for row in vector_input {
        if row.len() == 0 {
            continue;
        }
        let mut increasing: bool = false;
        let min_diff: i32 = 1;
        let max_diff: i32 = 3;
        let mut curr_el: i32 = -1;
        let mut first_el: bool = true;
        let mut second_el: bool = true;
        let mut incorrect_row: bool = false;
        for el in row {
            
            if first_el {
                curr_el = el;
                first_el = false;
                continue;
            }
            // check increase/decrease
            if second_el {
                second_el = false;
                if el < curr_el {
                    increasing = false;
                } else if el == curr_el {
                    incorrect_row = true;
                    break;
                } else {
                    increasing = true;
                }
            } else {
                if !((el < curr_el && !increasing) || (el > curr_el && increasing)) || (el == curr_el) {
                    incorrect_row = true;
                    break;
                }
            }
            // check range
            let abs_dif: i32 = (el - curr_el).abs();
            if (min_diff > abs_dif) || (max_diff < abs_dif) {
                incorrect_row = true;
                break;
            }
            curr_el = el;
        }
        if !incorrect_row {
            count_success += 1;

        }


    }
    dbg!(&count_success);
    // asd
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
