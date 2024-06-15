fn main() {
    let nums = [2, 7, 11, 15];
    let target = 26;

    // for (i, &oi) in nums.iter().enumerate() {
    //     for (j, &ni) in nums[i+1..].iter().enumerate() {
    //         println!("Items: {} == {}; i: {}; j: {}", oi, ni, i, j);
    //         if oi + ni == target {
    //             println!("Target {}: Indexes; ({}, {})", target, i, j);
    //         }
    //     }
    // }

    let mut index: usize = 0;
    'outer_loop: for oi in nums {
        for ni in &nums[index+1..] {
            println!("Items: {} == {}", oi, ni);
            if oi + ni == target {
                println!("Target {}: Indexes; ({}, {})", target, oi, ni);
                break 'outer_loop;
            }
        }
        index += 1;
    }
}
