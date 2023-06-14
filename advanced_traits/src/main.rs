// fully qualified syntax 
// <Type as Trait>::function(receiver_if_method, next_arg, ...);

// Same as string, person is just an "alias" for string.    
//
//
//
//
//
// fn generic<T: ?Sized>(t: &T) {
//     // --snip--
// }
// A trait bound on ?Sized means “T may or may not be Sized” and this notation 
// overrides the default that generic types must have a known size at 
// compile time. The ?Trait syntax with this meaning is only available for 
// Sized, not any other traits.
type Person = String;

fn never_return(num: i32) {
    match num {
        1..=3 => print!("1 or two"),
        _ => (),
    }
}

fn advanced_functions () {
        let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();

}

fn main() {
    println!("Hello, world!");
}
