fn calculate_length(s: &String) -> usize { // s is a reference to a String
    return s.len();
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// fn change_fail(s: &String) {
//     s.push_str(", World!");
// }

fn change_success(s: &mut String) {
    s.push_str(", World!");
}

fn main() {
    let mut s: String = String::from("Hello");
    let len: usize = calculate_length(&s);
    println!("The length of {s} is {len}");

    // This won't work since it tries to modify the contents
    // of the borrowed variable
    // change_fail(&s1); 

    change_success(&mut s);

    // Cannot have two mutable references to the same value
    // let r1: &mut String = &mut s;
    // let r2: &mut String = &mut s;
    // println!("{}", r2);
    // println!("{}", r1);

    // Conversely, we CAN have multiple immutable references at the same time,
    // but that also fails if a mutable reference is also declared in the same scope
    // let r1: &String = &s; // no problem
    // let r2: &String = &s; // no problem
    // let r3: &mut String = &mut s; // BIG PROBLEM


}
