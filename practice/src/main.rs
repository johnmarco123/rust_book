fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = list.iter()
        .enumerate()
        .collect();

    println!("{:?}", only_borrows);

    println!("Before calling closure: {:?}", list);
}
