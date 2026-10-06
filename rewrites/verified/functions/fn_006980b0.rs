// original: 0x006980b0 mover_rebind_from_source (proposed)

/// Notify the old target, then rebind this mover from `src`.
///
/// Calls the vtable slot-1 routine of the current object (`this+0`) with
/// `tag`, passing `this` as the object. Then `src` is recorded at `this+0x0c`
/// and twelve of its dwords are copied into the payload, skipping `src+0x0c`:
/// `src+0x00/04/08` to `this+0x10/14/18`, `src+0x10/14/18` to `this+0x20/24/28`,
/// `src+0x20/24/28` to `this+0x30/34/38`, `src+0x30/34/38` to `this+0x40/44/48`.
/// Returns the last word copied (`src+0x38`).
///
/// Original: thiscall, two stack words (`tag`, `src`), callee cleans 8.
lf_checker_rt::export!(thiscall, rw_006980b0(this: u32, tag: u32, src: u32) -> u32 {
    unsafe {
        const NOTIFY_SLOT: u32 = 4;
        const TAG_WORD: u32 = 0x0c;
        const PAIRS: [(u32, u32); 12] = [
            (0x10, 0x00),
            (0x14, 0x04),
            (0x18, 0x08),
            (0x20, 0x10),
            (0x24, 0x14),
            (0x28, 0x18),
            (0x30, 0x20),
            (0x34, 0x24),
            (0x38, 0x28),
            (0x40, 0x30),
            (0x44, 0x34),
            (0x48, 0x38),
        ];
        let vtable = (this as *const u32).read_unaligned();
        let routine =
            (vtable as *const u32).byte_offset(NOTIFY_SLOT as isize).read_unaligned();
        let notify: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(routine as usize);
        let _ = notify(this, tag);
        (this as *mut u32).byte_offset(TAG_WORD as isize).write_unaligned(src);
        let mut last: u32 = 0;
        for &(d, s) in &PAIRS {
            last = (src as *const u32).byte_offset(s as isize).read_unaligned();
            (this as *mut u32).byte_offset(d as isize).write_unaligned(last);
        }
        last
    }
});
