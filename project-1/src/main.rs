fn main() {
    let input = [11, 10, 11, 9, 7];
    
    let is_even = is_even(input);

    println!("{}", is_even)

    // if is_even {
    //     println!("{} is even", input)
    // } else {
    //     println!("{} is odd", input)
    // }
}

fn is_even(input: [i32; 5]) -> bool {
    let mut result = false;
    // let mut val = 0;
    
    for num in input.iter() {
        if num % 2 == 0 {
            result = true;
            // val = num
            break;
        } else {
            result = false;
            // val = num;
            break;
        }
    }
    
    result
}