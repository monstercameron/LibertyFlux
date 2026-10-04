// original: 0x008915d0 aud_vec_param_apply
/// Applies the parameter callee unless the vector is degenerate.
///
/// Sums the squares of the three floats at the argument pointer; when the sum
/// is NaN or infinite it returns 0x7f800000 without calling. Otherwise it
/// resolves the target like `aud_param_apply_flagged` and invokes the callee
/// with the target and the vector pointer. Returns the callee's answer.
///
/// Note: the original also overwrites its own incoming argument slot with the
/// sum bits; that clobber is dead after the callee-cleaned return, so the
/// contract disables the stack comparison (the same precedent as the bool
/// quirk lanes).
export!(thiscall, rw_008915d0(this: *mut u8, vec: *const f32) -> u32 {
    unsafe {
        let x = *vec;
        let y = *(vec.add(1));
        let z = *(vec.add(2));
        let s = x * x + y * y + z * z;
        if s.is_nan() || s.is_infinite() {
            return 0x7f800000;
        }
        let b = *(this.add(4));
        let target = if b == 0xff {
            0
        } else {
            let stride = *global::<u32>(0x115d968);
            let table = *global::<u32>(0x115d988);
            let idx = *(this.add(0x40)) as u32;
            let entry = *((table
                .wrapping_add(idx.wrapping_mul(0x6f40))
                .wrapping_add(0x6f14)) as *const u32);
            stride.wrapping_mul(b as u32).wrapping_add(entry)
        };
        callee_thiscall!(1, u32, target, vec as u32)
    }
});
