// original: 0x00A4C890 vehicle_remove_from_array (proposed)

/// Removes the argument from the slot array at `this + SLOTS`, releasing it.
///
/// A zero argument returns at once. When the word at `this + MODE` (0x1300)
/// is `FULL_MODE` (3) all eight slots are scanned, otherwise only the first
/// `count = byte[this + COUNT]` (0x1070), zero-extended (a zero count returns
/// at once). On a match the slot is released through the callee (`this` = old
/// value, argument = slot address; the old value is non-zero there by
/// construction) and zeroed. Returns nothing defined.
///
/// Original: 0x00A4C890 (thiscall, one stack word), one callee at two sites.
lf_checker_rt::export!(thiscall, rw_00A4C890(this: u32, val: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x0F54;
        const MODE: u32 = 0x1300;
        const COUNT: u32 = 0x1070;
        const FULL_MODE: u32 = 3;
        const FULL_COUNT: i32 = 8;
        const RELEASE_CALLEE: u32 = 1;
        if val == 0 {
            return 0;
        }
        let n = if ((this + MODE) as *const u32).read_unaligned() == FULL_MODE {
            FULL_COUNT
        } else {
            ((this + COUNT) as *const u8).read() as i32
        };
        if n <= 0 {
            return 0;
        }
        let mut i = 0;
        while i < n {
            let slot = (this + SLOTS + (i as u32) * 4) as *mut u32;
            if slot.read_unaligned() == val {
                let old = slot.read_unaligned();
                if old != 0 {
                    lf_checker_rt::callee_thiscall!(
                        RELEASE_CALLEE,
                        u32,
                        old,
                        slot as u32
                    );
                }
                slot.write_unaligned(0);
                return 0;
            }
            i += 1;
        }
        0
    }
});
