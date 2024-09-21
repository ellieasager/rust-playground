use std::collections::HashSet;

fn main() {
    // Define a vector of ranges
    let ranges = vec![[15, 18], [2, 6], [8, 10], [1, 3]];


    // Call merge_ranges function with the ranges
    let merged = merge_ranges(ranges);
    
    println!("Output: {:?}", merged);
    
    // let ranges = vec![[1, 5], [2, 3], [4, 8], [9, 10], [9, 12]];
    // Expected output: [1, 6], [8, 10], [15, 18]
}

fn merge_ranges(mut ranges: Vec<[i32; 2]>) -> Vec<[i32; 2]> {

    ranges.sort_by(|a, b| a.first().unwrap().cmp(b.first().unwrap()));

    // let a = HashSet::from([1, 2, 3]);
    // let b = HashSet::from([5, 4]);
    // let mut intersection = a.intersection(&b);
    // let result = intersection.count();


    // a b c


    // a b 

    // ab 

    let mut accum: Vec<[i32; 2]> = Vec::new();

    for (position, &range) in ranges.iter().enumerate() {

        if position == ranges.len() - 2{
            break;
        }

        let mut a = HashSet::from(range);
        if position > 0 {
            a = HashSet::from(*accum.last().unwrap());
        }

        let range_b = ranges[position + 1];
        let b = HashSet::from(range_b);
        let intersection = a.intersection(&b);

        // no overlap
        if intersection.count() == 0 {
            let a_vec = a.into_iter().collect::<Vec<_>>();
            let a_arr: [i32; 2] = [*a_vec.first().unwrap(),*a_vec.last().unwrap()];
            accum.push(a_arr);
            accum.push(range_b);


        // overlap
        } else {
            let a_vec = a.union(&b).into_iter().collect::<Vec<_>>();
            let a_arr: [i32; 2] = [**a_vec.first().unwrap(), **a_vec.last().unwrap()];
            accum.push(a_arr);
        }

    }

    accum
}