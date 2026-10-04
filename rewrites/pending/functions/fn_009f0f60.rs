// original: 0x009f0f60 ped_drain_work_list
/// Drain the work list behind the inner object at `0x78`.
///
/// Pulls entries until the list reports null. A fresh entry (word at +8
/// clear) is marked with bit `0x4000`, seeded with the 1000.0 and 0.0
/// parameters, and handed to the worker entry. Always returns 0.
export!(thiscall, rw_009f0f60(this_ptr: u32) -> u32 {
    unsafe {
        let inner = *((this_ptr + 0x78) as *const u32);
        let mut entry: u32 = callee_thiscall!(2, u32, inner);
        loop {
            if entry == 0 {
                return 0;
            }
            if *((entry + 8) as *const u32) == 0 {
                *((entry + 4) as *mut u32) |= 0x4000;
                let _: u32 = callee_thiscall!(3, u32, entry, 1000.0f32.to_bits());
                let _: u32 = callee_thiscall!(4, u32, entry, 0.0f32.to_bits());
                let inner_now = *((this_ptr + 0x78) as *const u32);
                let _: u32 = callee_thiscall!(5, u32, inner_now, entry);
            }
            let inner_next = *((this_ptr + 0x78) as *const u32);
            entry = callee_thiscall!(6, u32, inner_next);
        }
    }
});
