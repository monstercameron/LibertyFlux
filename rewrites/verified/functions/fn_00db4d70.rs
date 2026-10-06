// original: 0x00DB4D70 UIMouseCursor::vf81

/// Slot 81 of the mouse-cursor virtual table: visibility test.
///
/// `this` is the cursor object. A hook at slot `+0x140` of the object's
/// virtual table is called with the object; only the low byte of its
/// answer matters. The result is true when the hook's low byte is non-zero
/// and the flag byte at `NO_CURSOR` is clear and the flag byte at
/// `VISIBLE` is set. The hook runs scripted on both sides through a
/// planted table entry; the low byte of the return is compared (`al`: the
/// original sets only `al`, leaving the hook's high bytes in place).
///
/// Original: 0x00DB4D70 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00db4d70(this: u32) -> u32 {
    unsafe {
        const HOOK_SLOT: u32 = 0x140;
        const NO_CURSOR: u32 = 0xcb;
        const VISIBLE: u32 = 0x215;

        let vtable = (this as *const u32).read_unaligned();
        let hook: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vtable + HOOK_SLOT) as *const u32).read_unaligned() as usize);
        let answer = hook(this);
        if (answer & 0xff) == 0 {
            return 0;
        }
        if ((this + NO_CURSOR) as *const u8).read() != 0 {
            return 0;
        }
        if ((this + VISIBLE) as *const u8).read() == 0 {
            return 0;
        }
        1
    }
});
