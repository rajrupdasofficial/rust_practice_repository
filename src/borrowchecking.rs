/*
The Rust borrow checker is a fundamental feature of the Rust programming language that ensures memory safety without the need for a garbage collector. It enforces strict rules around ownership and borrowing, which can initially seem challenging but ultimately leads to safer and more efficient code.
Key Concepts of Borrowing in Rust
Ownership: In Rust, every value has a single owner at any given time. When the owner goes out of scope, the value is automatically dropped.
Borrowing: Instead of transferring ownership, Rust allows borrowing of values. There are two types of borrows:
Immutable Borrowing: You can have multiple immutable references to a value simultaneously. This means that while you can read the value, you cannot modify it.
Mutable Borrowing: You can have only one mutable reference to a value at a time. This ensures that while a value is being modified, no other references (mutable or immutable) can access it.
The Borrow Checker: This is the component of the Rust compiler that enforces the borrowing rules. It checks at compile time whether the borrowing rules are followed, preventing data races and ensuring memory safety.
*/
pub fn borrw_checking() {
    let s = String::from("Helloworld");
    //immutableborrow
    let len = calculate_length(&s);
    println!("immutable borrow checking");
    println!("The length of '{}' is {}.", s, len);
    //mutable borrow
    let mut s_mut = String::from("Helloworld");
    change(&mut s_mut);
    println!("Mutable borrowchecking");
    println!("After change : {}", s_mut);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
