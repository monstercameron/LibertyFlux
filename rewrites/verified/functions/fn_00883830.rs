// original: 0x00883830 stream_pool_release (proposed)
/// Release one pool slot: clear its bit in the allocation bitmap under the pool lock.
///
/// Takes the pool lock (intercepted callee 1, thiscall: scratch in `ecx`,
/// lock address as the stack argument), converts `ptr` to a slot index with
/// signed division by `SLOT_BYTES` (24) relative to the pool base global at
/// file address `0x1154088`, clears bit `index & 31` of word `index >> 5` in
/// the bitmap (global at `0x1154090`), and releases the lock (intercepted
/// callee 2, thiscall, scratch in `ecx`). The scratch area is two zero
/// words, matching the checker's zero stack fill.
///
/// Original: cdecl, one stack argument, no return value.
lf_checker_rt::export!(cdecl, rw_00883830(ptr: u32) -> u32 {
    unsafe {
        const POOL_LOCK: u32 = 0x0115_4068;
        const POOL_BASE: u32 = 0x0115_4088;
        const POOL_BITMAP: u32 = 0x0115_4090;
        const SLOT_BYTES: i32 = 24;
        const LOCK_CALLEE: u32 = 1;
        const UNLOCK_CALLEE: u32 = 2;
        let mut scratch = [0u32; 2];
        let lock = lf_checker_rt::relocated(POOL_LOCK);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOCK_CALLEE,
            u32,
            scratch.as_mut_ptr() as u32,
            lock
        );
        let base = (lf_checker_rt::global::<u32>(POOL_BASE) as *const u32).read_unaligned();
        let bitmap =
            (lf_checker_rt::global::<u32>(POOL_BITMAP) as *const u32).read_unaligned();
        let index = (ptr.wrapping_sub(base) as i32).wrapping_div(SLOT_BYTES);
        let word = bitmap.wrapping_add(((index >> 5) as u32).wrapping_mul(4));
        let bit = 1u32 << ((index & 31) as u32);
        let slot = word as *mut u32;
        slot.write_unaligned(slot.read_unaligned() & !bit);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(UNLOCK_CALLEE, u32, scratch.as_mut_ptr() as u32);
        0
    }
});
