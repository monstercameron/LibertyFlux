// original: 0x00a1d950 cam_chain_blend_add (proposed)

/// Chains two worker floats and adds a bias, returning the sum.
///
/// `a0` selects the chain input and `a1` is a float bias (bits). The
/// first worker callee runs on (`this`, `a0`) and yields `f1`; the second
/// runs on (`this`, slot) where slot starts as `f1` and comes back as
/// `f2`. The result is `f2 + bias` in that operand order, returned on
/// `st0`. (The original threads the slot through its own incoming stack
/// frame, which a Rust rewrite cannot address; the rewrite uses a local
/// instead, the call comparison skips that address while comparing the
/// pointed-to words, and the stack check is off — the values are still
/// observed through the call snapshots and the return channel.)
///
/// Original: 0x00a1d950 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a1d950(this: u32, a0: u32, a1: u32) -> f32 {
    unsafe {
        const C1: u32 = 1;
        const C2: u32 = 2;
        let f1: f32 = lf_checker_rt::callee_thiscall!(C1, f32, this, a0);
        let mut slot = f1.to_bits();
        lf_checker_rt::callee_thiscall!(C2, u32, this, &mut slot as *mut u32 as u32);
        let f2 = f32::from_bits(slot);
        core::hint::black_box(f2) + core::hint::black_box(f32::from_bits(a1))
    }
});
