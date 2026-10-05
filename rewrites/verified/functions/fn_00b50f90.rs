// original: 0x00b50f90 retain_all_events (proposed)

/// Add one reference to every event in a group list.
///
/// `this` points to the list: signed count at `+0x08`, event pointers from
/// `+0x0C`. Each event's reference count at `+0x04` is incremented. A
/// non-positive count does nothing. No meaningful return value.
///
/// Original: 0x00b50f90 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b50f90(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x08;
        const ITEMS: u32 = 0x0c;
        const REFCOUNT: u32 = 0x04;
        let count = ((this + COUNT) as *const i32).read_unaligned();
        if count > 0 {
            let mut i: i32 = 0;
            while i < count {
                let ev = ((this + ITEMS + (i as u32) * 4) as *const u32).read_unaligned();
                let rc = (ev + REFCOUNT) as *mut u32;
                rc.write_unaligned(rc.read_unaligned().wrapping_add(1));
                i += 1;
            }
        }
        0
    }
});
