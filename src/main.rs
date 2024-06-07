fn find_unique_elements_increase(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
   
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);


    while i < arr1.len() && j < arr2.len() {

        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else if arr1[i] > arr2[j] {
            result.push(arr2[i]);
            j += 1;
        } else {
            i += 1;
            j += 1;
        }
    }

    result.extend_from_slice(&arr1[i..]);
    result.extend_from_slice(&arr2[j..]);

    result
}

fn find_unique_elements_decrease(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] > arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else if arr1[i] < arr2[j] {
            result.push(arr2[j]);
            j += 1;
        } else {
            i += 1;
            j += 1;
        }
    }

    result.extend_from_slice(&arr1[i..]);
    result.extend_from_slice(&arr2[j..]);

    result
}

fn deduplicate(result: &[i32]) -> (Vec<i32>, bool){
    if result.len() == 0 {
        return (Vec::new(), false);
    }

    let mut deduplicated = vec![result[0]];
    let mut increase = false;


    let mut last_element = result[0];
    let mut i = 1;
    while i < result.len() {
        if result[i] == last_element{
            i += 1;
            continue
        }
        increase = result[i] > last_element;
        deduplicated.push(result[i]);
        last_element = result[i];
        i += 1;
    }

    (deduplicated, increase)
}

fn revert(result: &[i32]) -> Vec<i32>{
    let mut i = (result.len() - 1) as i32;
    let mut output = Vec::new();
    while i >= 0 {
        output.push(result[i as usize]);
        i -= 1;
    }
    output
}

pub fn get_array(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let (deduplicated_arr1, increase1) = deduplicate(&arr1);
    let (deduplicated_arr2, increase2) = deduplicate(&arr2);

    match (increase1, increase2) {
        (true, true) => {
            println!("{:?}", deduplicated_arr1);
            println!("{:?}", deduplicated_arr2);
            find_unique_elements_increase(&deduplicated_arr1, &deduplicated_arr2)
        },
        (false, false) => {
            println!("{:?}", deduplicated_arr1);
            println!("{:?}", deduplicated_arr2);
            find_unique_elements_decrease(&deduplicated_arr1, &deduplicated_arr2)
        },
        (true, false) => {
            println!("{:?}", &deduplicated_arr1);
            println!("{:?}", revert(&deduplicated_arr2));
            find_unique_elements_increase(&deduplicated_arr1, &revert(&deduplicated_arr2))
        },
        (false, true) => {
            println!("{:?}", &revert(&deduplicated_arr1));
            println!("{:?}", &deduplicated_arr2);
            find_unique_elements_increase(&revert(&deduplicated_arr1), &deduplicated_arr2)
        },
    }

}

fn main() {
    // let arr1 = vec![1, 2, 3, 4, 5];
    // let arr2 = vec![3, 4, 5, 6, 6, 7];
    //
    // println!("Unique elements increase: {:?}", get_array(&arr1, &arr2));
    //
    // let arr1 = vec![5, 4, 3, 2, 1];
    // let arr2 = vec![7, 6, 6, 5, 4, 3];
    //
    // println!("Unique elements decrease: {:?}", get_array(&arr1, &arr2));
    //
    // let arr1 = vec![5, 4, 3, 2, 1];
    // let arr2 = vec![3, 4, 5, 6, 6, 7];
    //
    // println!("Unique elements decrease: {:?}", get_array(&arr1, &arr2));
    //

    let arr1 = vec![3, 4, 5, 6, 6, 7];
    let arr2 = vec![5, 4, 3, 2, 1];

    println!("Unique elements decrease: {:?}", get_array(&arr1, &arr2));

}
