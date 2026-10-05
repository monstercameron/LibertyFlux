// original: 0x00b4ef60 reset_slot_118h (proposed)

/// Reset the helper object in slot 0x118 and release it.
///
/// `this + 0x118` holds a helper pointer (or null, which does nothing).
/// The helper's virtual slot at `+0xB0` runs first (thiscall on the helper),
/// then its mode word at `+0x224` is set to 1, its sub-mode byte at `+0x22A`
/// to 3, and flag bits 0 and 5 at `+0x24` are cleared. Finally the helper is
/// released (callee 2, thiscall on the helper, passed the slot address) and
/// the slot is cleared. No meaningful return value.
///
/// Original: 0x00b4ef60 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b4ef60(this: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x118;
        const RESET_SLOT: u32 = 0xb0;
        const RELEASE: u32 = 2;
        const MODE: u32 = 0x224;
        const SUBMODE: u32 = 0x22a;
        const FLAGS: u32 = 0x24;
        let slot = this + SLOT;
        let helper = (slot as *const u32).read_unaligned();
        if helper == 0 {
            return 0;
        }
        let vt = (helper as *const u32).read_unaligned();
        let reset: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt + RESET_SLOT) as *const u32).read_unaligned() as usize);
        reset(helper);
        let o = (slot as *const u32).read_unaligned();
        ((o + SUBMODE) as *mut u8).write(3);
        let o = (slot as *const u32).read_unaligned();
        ((o + MODE) as *mut u32).write_unaligned(1);
        let o = (slot as *const u32).read_unaligned();
        let flags = (o + FLAGS) as *mut u32;
        flags.write_unaligned(flags.read_unaligned() & !1u32);
        let o = (slot as *const u32).read_unaligned();
        let flags = (o + FLAGS) as *mut u32;
        flags.write_unaligned(flags.read_unaligned() & !0x20u32);
        let inner = (slot as *const u32).read_unaligned();
        if inner != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, inner, slot);
        }
        (slot as *mut u32).write_unaligned(0);
        0
    }
});
