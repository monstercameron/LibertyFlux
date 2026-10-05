// original: 0x00988540 audCutscene_ctor (proposed)

/// Cutscene audio entity constructor: chains the base constructor, installs
/// the vtable and zeroes the slot fields.
///
/// Runs the base constructor (callee id 1) on `this`, writes the vtable
/// pointer `VTABLE` at `+0x00`, then twice (slots 0 and 1): zeroes the dword
/// at `+SLOT_PTR_OFF + 4 * slot` and the byte at `+SLOT_BYTE_OFF + 0x40 * slot`.
/// Finally zeroes the dword at `+0x10` and the byte at `+0xa0`, and returns `this`.
/// Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00988540(this: u32) -> u32 {
    const VTABLE: u32 = 0xe8e4b4;
    const BASE_CTOR: u32 = 1;
    const SLOT_PTR_OFF: u32 = 8;
    const SLOT_BYTE_OFF: u32 = 0x14;
    const SLOT_BYTE_STRIDE: u32 = 0x40;
    unsafe {
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        ((this + 0x00) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE));
        for slot in 0..2u32 {
            ((this + SLOT_BYTE_OFF + slot * SLOT_BYTE_STRIDE) as *mut u8).write(0);
            ((this + SLOT_PTR_OFF + slot * 4) as *mut u32).write_unaligned(0);
        }
        ((this + 0x10) as *mut u32).write_unaligned(0);
        ((this + 0xa0) as *mut u8).write(0);
    }
    this
});
