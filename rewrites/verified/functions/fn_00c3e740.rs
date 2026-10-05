// original: 0x00c3e740 train_build_frame_via_helpers (proposed)
/// Run two helpers over this car and two float arguments.
///
/// `this` (ECX) is the car, `f0`/`f1` are float bit patterns. Calls
/// helper id 1 (cdecl: scratch pointer, f0, f1) with the floats passed by
/// value, then helper id 2 (thiscall/1: a second scratch pointer) on this
/// car. The scratch pointers address the original's own stack frame, so
/// the contract skips them and compares only the float words; the rewrite
/// passes pointers to its own locals. Returns helper 2's answer.
///
/// Original: 0x00c3e740 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c3e740(this: u32, f0bits: u32, f1bits: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 1;
        const SECOND: u32 = 2;
        let mut scratch = [0u32; 8];
        let p0 = scratch.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(FIRST, u32, p0, f0bits, f1bits);
        // The original's second scratch pointer sits 6 bytes past the
        // first; the value is skipped by the contract either way.
        let got: u32 = lf_checker_rt::callee_thiscall!(SECOND, u32, this, p0.wrapping_add(6));
        got
    }
});
