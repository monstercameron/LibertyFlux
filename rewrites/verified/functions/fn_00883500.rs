// original: 0x00883500 stream_pool_alloc (proposed)
/// Allocate one pool slot: find a free bit, set it, and return the cleared object.
///
/// Takes the pool lock (intercepted callee 1, thiscall: scratch in `ecx`,
/// lock address as the stack argument). When the slot count (half-word
/// global at file address `0x1154096`) is positive, scans the allocation
/// bitmap (global at `0x1154090`) from index 0 for the first clear bit; on a
/// hit sets the bit, zeroes the four header words of slot `i` (`base + i *
/// 24`, base from the global at `0x1154088`), and returns the slot address.
/// With no free bit (or a non-positive count) returns null. The lock is
/// released on every path (intercepted callee 2, thiscall, scratch in
/// `ecx`); the scratch area is two zero words, matching the checker's zero
/// stack fill.
///
/// Original: cdecl, no stack arguments, returns the slot (or null) in `eax`.
lf_checker_rt::export!(cdecl, rw_00883500() -> u32 {
    unsafe {
        const POOL_LOCK: u32 = 0x0115_4068;
        const POOL_BASE: u32 = 0x0115_4088;
        const POOL_BITMAP: u32 = 0x0115_4090;
        const POOL_COUNT: u32 = 0x0115_4096;
        const SLOT_BYTES: u32 = 24;
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
        let count =
            (lf_checker_rt::global::<u16>(POOL_COUNT) as *const u16).read_unaligned() as i32;
        let bitmap =
            (lf_checker_rt::global::<u32>(POOL_BITMAP) as *const u32).read_unaligned();
        let base = (lf_checker_rt::global::<u32>(POOL_BASE) as *const u32).read_unaligned();
        let mut slot = 0u32;
        if count > 0 {
            let mut i = 0i32;
            while i < count {
                let word = (bitmap.wrapping_add(((i >> 5) as u32).wrapping_mul(4))) as *mut u32;
                let bit = 1u32 << ((i & 31) as u32);
                if word.read_unaligned() & bit == 0 {
                    word.write_unaligned(word.read_unaligned() | bit);
                    let addr = base.wrapping_add((i as u32).wrapping_mul(SLOT_BYTES));
                    (addr as *mut u32).write_unaligned(0);
                    ((addr + 4) as *mut u32).write_unaligned(0);
                    ((addr + 12) as *mut u32).write_unaligned(0);
                    ((addr + 8) as *mut u32).write_unaligned(0);
                    slot = addr;
                    break;
                }
                i += 1;
            }
        }
        let _: u32 =
            lf_checker_rt::callee_thiscall!(UNLOCK_CALLEE, u32, scratch.as_mut_ptr() as u32);
        slot
    }
});
