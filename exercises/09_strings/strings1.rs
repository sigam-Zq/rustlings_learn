// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

// -----I AM NOT DONE

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
    // println!("My current favorite color is {:?}",  "answer");
    print_type_of(&"aaaa");
    print_type_of(&answer)
}

fn current_favorite_color() -> String {
    String::from("blue")
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}