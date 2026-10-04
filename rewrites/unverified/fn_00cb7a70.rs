// original: 0x00cb7a70 move_init_with_limit
/// Initialise a move task through two callees, then clamp (2 calls).
///
/// Runs the finder callee on the object from the global at file address
/// `0x0179D114` (thiscall, five stack arguments; `a0` and `a1` are unread
/// except through a skipped slot). A non-null answer is zeroed at its
/// first word and stored at `[this + 0x34]` (else zero is stored). Then
/// runs the four-argument initialiser with (`this`, T, T, `a2`, 1), where
/// the first two slots are read half-overlapping the return address and
/// below-stack scratch and are skipped by the contract. Stores `a4` at
/// `[this + 0x38]`, stores `a3` at `[this + 0x28]` when it is not below
/// `0.0` (negative and NaN become `0.5`), and returns `a4`. Both callees
/// are intercepted by the checker.
lf_checker_rt::export!(thiscall, rw_00cb7a70(this: u32, _a0: u32, _a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        use lf_checker_rt::global;
        /// Global holding the finder callee's object (file VA).
        const FINDER_OBJ_G: u32 = 0x0179D114;
        /// Slots for the found pointer, the limit and the clamped value.
        const FOUND_OFF: u32 = 0x34;
        const LIMIT_OFF: u32 = 0x38;
        const CLAMP_OFF: u32 = 0x28;
        /// Clamp replacement for a negative or NaN limit.
        const CLAMP_DEFAULT: f32 = 0.5;
        const FIND: u32 = 1;
        const INIT: u32 = 2;
        let c = *global::<u32>(FINDER_OBJ_G);
        let r: u32 = lf_checker_rt::callee_thiscall!(FIND, u32, c);
        if r != 0 {
            (r as *mut u32).write_unaligned(0);
            ((this + FOUND_OFF) as *mut u32).write_unaligned(r);
        } else {
            ((this + FOUND_OFF) as *mut u32).write_unaligned(0);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, this, 0, 0, a2, 1);
        ((this + LIMIT_OFF) as *mut u32).write_unaligned(a4);
        let x = f32::from_bits(a3);
        if x >= 0.0 {
            ((this + CLAMP_OFF) as *mut u32).write_unaligned(a3);
        } else {
            ((this + CLAMP_OFF) as *mut u32).write_unaligned(CLAMP_DEFAULT.to_bits());
        }
        a4
    }
});
