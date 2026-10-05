// original: 0x00883890 stream_aux_release (proposed)
/// Release one auxiliary slot: clear its bit in the auxiliary bitmap under lock.
///
/// Takes the auxiliary lock (intercepted callee 1, thiscall: scratch in
/// `ecx`, lock address as the stack argument), converts `ptr` to a slot
/// index as `(ptr - base) / 32` relative to the auxiliary base global at
/// file address `0x1154064` (an arithmetic shift right by 2 followed by a
/// logical shift right by 3, reproduced exactly), clears bit `index & 31`
/// of word `index >> 5` in the bitmap (global at `0x1154098`), and releases
/// the lock (intercepted callee 2, thiscall, scratch in `ecx`). The scratch
/// area is two zero words, matching the checker's zero stack fill.
///
/// Original: cdecl, one stack argument, no return value.
lf_checker_rt::export!(cdecl, rw_00883890(ptr: u32) -> u32 {
    unsafe {
        const AUX_LOCK: u32 = 0x0115_40a0;
        const AUX_BASE: u32 = 0x0115_4064;
        const AUX_BITMAP: u32 = 0x0115_4098;
        const LOCK_CALLEE: u32 = 1;
        const UNLOCK_CALLEE: u32 = 2;
        let mut scratch = [0u32; 2];
        let lock = lf_checker_rt::relocated(AUX_LOCK);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOCK_CALLEE,
            u32,
            scratch.as_mut_ptr() as u32,
            lock
        );
        let base = (lf_checker_rt::global::<u32>(AUX_BASE) as *const u32).read_unaligned();
        let bitmap =
            (lf_checker_rt::global::<u32>(AUX_BITMAP) as *const u32).read_unaligned();
        let shifted = (ptr.wrapping_sub(base) as i32 >> 2) as u32 >> 3;
        let word = bitmap.wrapping_add((shifted >> 5).wrapping_mul(4));
        let bit = 1u32 << (shifted & 31);
        let slot = word as *mut u32;
        slot.write_unaligned(slot.read_unaligned() & !bit);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(UNLOCK_CALLEE, u32, scratch.as_mut_ptr() as u32);
        0
    }
});
