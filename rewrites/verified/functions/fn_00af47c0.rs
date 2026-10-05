// original: 0x00AF47C0 drawable_dispatch_mode (proposed)

/// Dispatch to the helper's table by the mode word's bits.
///
/// Loads the helper object at `this+0`; when null returns at once. Bit 1
/// of `mode` selects table slot 0x30 in mode 1 with (a1, a2, a4, 1, a3);
/// otherwise a nonzero mode selects slot 0x30 in mode 2 with (a1, a2, a4,
/// 2, a3); a zero mode selects slot 0x28 with (a1, a2, a3, a4). The helper
/// is the object in every case. Returns nothing meaningful.
///
/// Original: 0x00AF47C0 (thiscall, five stack words, two table-indirect callees).
lf_checker_rt::export!(thiscall, rw_00af47c0(this: u32, a1: u32, a2: u32, a3: u32, a4: u32, mode: u32) -> () {
    unsafe {
        const SLOT_WIDE: u32 = 0x30;
        const SLOT_NARROW: u32 = 0x28;
        const MODE_MASK: u32 = 2;
        let obj = ((this) as *const u32).read_unaligned();
        if obj == 0 {
            return;
        }
        let vt = ((obj) as *const u32).read_unaligned();
        if mode & MODE_MASK != 0 {
            let tgt = ((vt.wrapping_add(SLOT_WIDE)) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(obj, a1, a2, a4, 1, a3);
        } else if mode != 0 {
            let tgt = ((vt.wrapping_add(SLOT_WIDE)) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(obj, a1, a2, a4, 2, a3);
        } else {
            let tgt = ((vt.wrapping_add(SLOT_NARROW)) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(obj, a1, a2, a3, a4);
        }
    }
});
