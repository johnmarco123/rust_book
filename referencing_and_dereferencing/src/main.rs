fn add_one(f: &mut Vec<i32>) {
    f.iter_mut().for_each(|x| *x += 1)
}

fn main() {
    let mut f = vec!(1, 2, 3, 4, 5);
    add_one(&mut f);
}
