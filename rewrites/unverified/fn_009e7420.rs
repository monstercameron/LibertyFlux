// original: 0x009e7420 ped_best_entry_score
/// Scores the five entries at `[this+0x224]+0x44..0x54` through
/// virtual slot `+0x18` (x87 float result): a negative score aborts
/// with 0, otherwise the maximum is kept at the out-pointer and 1 is
/// reported when at least one entry was live. (thiscall, 1 arg.)
lf_checker_rt::export!(thiscall, rw_009e7420(this_ptr: u32, out_ptr: u32) -> u32 {
    unsafe {
        const SLOTS_OFF: u32 = 0x224;
        const FIRST_OFF: u32 = 0x44;
        const ENTRY_COUNT: u32 = 5;
        const SLOT: u32 = 0x18;
        let slots = (this_ptr.wrapping_add(SLOTS_OFF) as *const u32).read_unaligned();
        let mut any = false;
        for i in 0..ENTRY_COUNT {
            let entry = (slots.wrapping_add(FIRST_OFF + i * 4) as *const u32).read_unaligned();
            if entry == 0 {
                continue;
            }
            let vt = (entry as *const u32).read_unaligned();
            let slot = (vt.wrapping_add(SLOT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32) -> f32 =
                unsafe { core::mem::transmute(slot as usize) };
            let x = f(entry, this_ptr);
            if x < 0.0 {
                return 0;
            }
            if !any {
                (out_ptr as *mut u32).write_unaligned(x.to_bits());
                any = true;
            } else {
                let cur = f32::from_bits((out_ptr as *const u32).read_unaligned());
                if x > cur {
                    (out_ptr as *mut u32).write_unaligned(x.to_bits());
                }
            }
        }
        if any { 1 } else { 0 }
    }
});
