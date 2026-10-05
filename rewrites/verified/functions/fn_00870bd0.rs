// original: 0x00870BD0 crmt_slot_array_init (proposed)

/// Initialise the 64-entry slot array at `this` (stride 0x34, 0x1000 bytes total).
///
/// Each slot `i` (base `this + i * 0x34`) is prepared the same way: the words
/// at offsets +4 and +8 are zeroed, the vtable word at +0 is set to the
/// pre-init table, the member initialiser (callee 1) runs with its object at
/// slot +12, then the vtable word is switched to the live table and the word
/// at +0x30 is zeroed. The loop counter runs from 63 down to 0 and is
/// compared as signed (continues while non-negative), so exactly 64 slots are
/// initialised. Returns `this`.
///
/// Original: 0x00870BD0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00870bd0(this: u32) -> u32 {
    const SLOT_COUNT: u32 = 64;
    const SLOT_STRIDE: u32 = 0x34;
    const MEMBER_OFF: u32 = 12;
    const TAIL_OFF: u32 = 0x30;
    unsafe {
        let mut slot = this;
        let mut remaining = SLOT_COUNT;
        while remaining != 0 {
            ((slot + 4) as *mut u32).write_unaligned(0);
            ((slot + 8) as *mut u32).write_unaligned(0);
            (slot as *mut u32)
                .write_unaligned(lf_checker_rt::relocated(0x00fe82c0));
            lf_checker_rt::callee_thiscall!(1, u32, slot + MEMBER_OFF);
            (slot as *mut u32)
                .write_unaligned(lf_checker_rt::relocated(0x00fe82d8));
            ((slot + TAIL_OFF) as *mut u32).write_unaligned(0);
            slot += SLOT_STRIDE;
            remaining -= 1;
        }
    }
    this
});
