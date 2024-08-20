use std::arch::asm;

pub fn inlineassembly() {
    // Multiply x by 6 using shifts and adds
    let mut x: u64 = 4;
    unsafe {
        asm!(
            "mov {tmp}, {x}",  // Move original x to tmp
            "shl {x}, 2",      // Shift x left by 2 (x * 4)
            "add {x}, {tmp}",  // Add original x (tmp) to x (x * 4 + x = x * 5)
            "add {x}, {tmp}",  // Add original x (tmp) again (x * 5 + x = x * 6)
            x = inout(reg) x,  // x is both input and output
            tmp = out(reg) _,  // tmp is output only
        );
    }

    // Print the output
    println!("The result of multiplying 4 by 6 is: {}", x);

    // Check if x is equal to 24
    assert_eq!(x, 4 * 6); // This will panic if the assertion fails
}
