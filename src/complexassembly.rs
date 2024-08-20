use std::arch::asm; // Import for inline assembly

// Function to find the pattern in the text
pub fn patternsolving() {
    let text = "Hello, world!";
    let pattern = "world";

    let found = contains_pattern(text, pattern);
    if found {
        println!("Pattern found!");
    } else {
        println!("Pattern not found.");
    }
}

// Function to check if the pattern exists in the text
fn contains_pattern(text: &str, pattern: &str) -> bool {
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let text_len = text_bytes.len();
    let pattern_len = pattern_bytes.len();

    let mut found: u8 = 0; // Use u8 instead of bool

    unsafe {
        asm!(
            "mov {text_len}, r8", // Load text length into r8
            "mov {pattern_len}, r9", // Load pattern length into r9
            "xor rax, rax", // Clear rax for the loop counter
            "0:", // Start of loop
            "cmp rax, r8", // Compare counter with text length
            "jge 2f", // If counter >= text length, exit loop
            "mov rdi, {text_ptr}", // Load text pointer
            "add rdi, rax", // Move to the current character in text
            "mov rsi, {pattern_ptr}", // Load pattern pointer
            "mov rdx, {pattern_len}", // Load pattern length
            "rep cmpsb", // Compare bytes in text and pattern
            "jne 1f", // If not equal, continue loop
            "mov byte ptr {found}, 1", // Set found to true
            "jmp 2f", // Exit loop
            "1:", // Continue loop
            "inc rax", // Increment loop counter
            "jmp 0b", // Repeat loop
            "2:", // End of loop
            found = out(reg_byte) found,
            text_ptr = in(reg) text_bytes.as_ptr(),
            pattern_ptr = in(reg) pattern_bytes.as_ptr(),
            text_len = in(reg) text_len,
            pattern_len = in(reg) pattern_len,
            options(nostack)
        );
    }

    found != 0 // Return true if found is non-zero, false otherwise
}
