// @link Problem definition [[docs/hackerrank/interview_preparation_kit/arrays/minimum_swaps_2.md]]

pub fn minimum_swaps(group: &[i32]) -> i32 {

    let mut indexed_group: Vec<_> = group.iter().map(|&i| i - 1).collect();

    let mut swaps = 0;
    let mut index: usize = 0;
    let size: usize = indexed_group.len();
    let mut temp: i32;
    let mut utemp: usize;

    while index < size {
      if indexed_group[index] == index.try_into().unwrap() {
        index += 1;
      } else {
        temp = indexed_group[index];
        utemp = temp.try_into().unwrap();
        indexed_group[index] = indexed_group[utemp];
        indexed_group[utemp] = temp;
        swaps += 1
      }
    }

    swaps
}
