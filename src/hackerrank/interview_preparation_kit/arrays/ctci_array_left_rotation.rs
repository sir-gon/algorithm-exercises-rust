// @link Problem definition [[docs/hackerrank/interview_preparation_kit/arrays/ctci_array_left_rotation.md]]

#[allow(non_snake_case)]
pub fn rotLeft(a: &[i32], d: i32) -> Vec<i32> {
    let mut result = a.to_vec();
    let rotations = d as usize % a.len();

    for i in 0..a.len() {
        let new_position = (i + a.len() - rotations) % a.len();
        result[new_position] = a[i];
    }

    result
}
