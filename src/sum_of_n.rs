use std::arch::asm;
pub fn sum_of_n(n: u32) -> u32 {
    let mut result: u32;
    unsafe {
        asm!(
            "mov eax, {0:e}",      // Move n into eax (32-bit format)
            "inc eax",             // eax = eax + 1 (calculates n + 1)
            "mul eax",             // eax = eax * n (calculates (n + 1) * n)
            "shr eax, 1",          // eax = eax / 2 (right shift by 1 to divide by 2)
            "mov {1:e}, eax",      // Move result from eax to the result variable
            in(reg) n,
            out(reg) result,
            options(nostack)       // No stack operations
        );
    }
    result
}
