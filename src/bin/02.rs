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

fn drop_element(row: Vec<i32>, position: u32) -> Vec<i32> {
    let mut new_row: Vec<i32> = Vec::new();
    let mut i: u32 = 0;

    for el in row {
        if i != position{
            new_row.push(el);
        }
        i += 1;
    }

    new_row
}

// fn run_tests(row: Vec<i32>, tolerance: u32, el1: i32, el2: i32, el3: i32) -> bool {
//     let incorrect_row: bool;
//     if tolerance > 0 {
//         dbg!(&row);
//         dbg!(&el1);
//         dbg!(&el2);
//         let row1: Vec<i32> = drop_element(row.clone(), el1);
//         let row2: Vec<i32> = drop_element(row.clone(), el2);

//         let row1_bool: bool = test_row(row1, tolerance - 1);
//         dbg!(&row1_bool);
//         let row2_bool: bool = test_row(row2, tolerance - 1);
//         dbg!(&row2_bool);

//         let row3_bool: bool;
//         if el3 != -1 {
//             dbg!(&el3);
//             let row3: Vec<i32> = drop_element(row.clone(), el3);
//             row3_bool = test_row(row3, tolerance - 1);
//             dbg!(&row3_bool);
//         } else {
//             row3_bool = true; 
//         }


//         incorrect_row = !(!row1_bool || !row2_bool || !row3_bool);
//         dbg!(&(row1_bool || row2_bool));
//         dbg!(&incorrect_row);
//     } else {
//         incorrect_row = true;
//     }

//     incorrect_row
// }


fn test_row(row: Vec<i32>) -> bool {
    let mut increasing: bool = false;
    let min_diff: i32 = 1;
    let max_diff: i32 = 3;
    let mut prev_el: i32 = -1;
    let mut first_el: bool = true;
    let mut second_el: bool = true;
    let mut incorrect_row: bool = false;
    for el in row {
        
        if first_el {
            prev_el = el;
            first_el = false;
            continue;
        }
        // check increase/decrease
        if second_el {
            second_el = false;
            if el < prev_el {
                increasing = false;
            } else if el == prev_el {
                incorrect_row = true;
                break;
            } else {
                increasing = true;
            }
        } else {
            if !((el < prev_el && !increasing) || (el > prev_el && increasing)) || (el == prev_el) {
                incorrect_row = true;
                break;
            }
        }
        // check range
        let abs_dif: i32 = (el - prev_el).abs();
        
        if (min_diff > abs_dif) || (max_diff < abs_dif) {
            incorrect_row = true;
            break;
        }
        prev_el = el;
    }

    incorrect_row
}

pub fn part_one(input: &str) -> Option<u32> {

    let vector_input: Vec<Vec<i32>> = read(input);
    let mut count_success: u32 = 0;
    for row in vector_input {
        if row.len() == 0 {
            continue;
        }
        if !test_row(row) {
            count_success += 1;
        }
    }

    Some(count_success)
}

pub fn test_row_tolerance(row: Vec<i32>) -> bool {
    let mut incorrect_row: bool = test_row(row.clone());
    for i in 0..=row.len() {
        if incorrect_row {
            incorrect_row = test_row(drop_element(row.clone(), i as u32))
        }
    }
    incorrect_row
}

pub fn part_two(input: &str) -> Option<u32> {
    let vector_input: Vec<Vec<i32>> = read(input);
    let mut count_success: u32 = 0;
    for row in vector_input {
        if row.len() == 0 {
            continue;
        }
        if !test_row_tolerance(row) {
            count_success += 1;
        }
    }
    Some(count_success)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
