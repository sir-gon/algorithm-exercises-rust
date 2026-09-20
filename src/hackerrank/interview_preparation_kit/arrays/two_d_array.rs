// @link Problem definition [[docs/hackerrank/interview_preparation_kit/arrays/two_d_array.md]]

use std::vec;

#[allow(non_snake_case)]
pub fn getHourglass(arr: Vec<Vec<i32>>, position_x: i32, position_y: i32) -> Vec<Vec<i32>> {
    let pos_x = position_x as usize;
    let pos_y = position_y as usize;

    vec![
        vec![arr[pos_x - 1][pos_y - 1], arr[pos_x - 1][pos_y], arr[pos_x - 1][pos_y + 1]],
        vec![arr[pos_x][pos_y]],
        vec![arr[pos_x + 1][pos_y - 1], arr[pos_x + 1][pos_y], arr[pos_x + 1][pos_y + 1]],
    ]
}

#[allow(non_snake_case)]
pub fn hourglassSum(arr: &[Vec<i32>]) -> i32 {
    let matrix_size: usize = arr.len();

    let matrix_start_index: usize = 1;
    let matrix_stop_index: usize = matrix_size - 1;

    let mut max_hourglass_sum: Option<i32> = None;

    for i in matrix_start_index..matrix_stop_index {
        for j in matrix_start_index..matrix_stop_index {
            let hourglass_values: Vec<Vec<i32>> = getHourglass(arr.to_vec(), i as i32, j as i32);
            let this_hourglass_sum: i32 = hourglass_values.iter().flatten().sum();

            if max_hourglass_sum.is_none() || this_hourglass_sum > max_hourglass_sum.unwrap() {
                max_hourglass_sum = Some(this_hourglass_sum);
            }
        }
    }

    max_hourglass_sum.unwrap()
}
