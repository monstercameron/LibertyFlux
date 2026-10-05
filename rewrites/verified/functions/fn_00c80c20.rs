// original: 0x00c80c20 scenario_vec_copy (proposed)

/// Copy the three-word vector at `this+0xa0` into the caller buffer `dst`.
///
/// Reads dwords at offsets `0xa0`, `0xa4`, `0xa8` of the `this` object (ECX)
/// and stores them at `dst[0..3]`. The two further stack words are popped but
/// never read. Returns `dst` (EAX still holds the first stack argument).
///
/// Original: thiscall, three stack words (the callee pops 0xc bytes).
lf_checker_rt::export!(thiscall, rw_00c80c20(this: u32, dst: u32, _u1: u32, _u2: u32) -> u32 {
    unsafe {
        const VEC_OFF: u32 = 0xa0;
        let x = ((this + VEC_OFF) as *const u32).read_unaligned();
        let y = ((this + VEC_OFF + 4) as *const u32).read_unaligned();
        let z = ((this + VEC_OFF + 8) as *const u32).read_unaligned();
        (dst as *mut u32).write_unaligned(x);
        ((dst + 4) as *mut u32).write_unaligned(y);
        ((dst + 8) as *mut u32).write_unaligned(z);
        dst
    }
});
