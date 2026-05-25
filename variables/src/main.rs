fn main() {
    for elm in 1..10 {
        println!("elems are: {elm}")
    }

    for elm in (1..10).rev() {
        println!("elems are: {elm}")
    }
}