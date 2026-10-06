// original: 0x00D785A0 classify_signed_field (proposed)

/// Classify the sign-extended word at `obj + 0x2e` under mode 0x2a.
///
/// Passes the field (sign-extended to 32 bits) and the mode constant to
/// the classifier and returns whether its low byte is non-zero, keeping
/// the answer's upper bytes (`setne al` only). Cdecl, one stack word.
use lf_checker_rt::{callee_cdecl, export};

const CLASSIFY: u32 = 1;

export!(cdecl, rw_00d785a0(obj: u32) -> u32 {
    unsafe {
        const FIELD_OFF: u32 = 0x2e;
        const MODE: u32 = 0x2a;
        let v = ((obj + FIELD_OFF) as *const i16).read_unaligned() as i32 as u32;
        let ans = callee_cdecl!(CLASSIFY, u32, v, MODE);
        (ans & 0xffff_ff00) | u32::from(ans & 0xff != 0)
    }
});
