// original: 0x00D01440 CEventCallForCover::vf21

/// Clone a call-for-cover event through the event manager.
///
/// Null from the manager (patched callee 1) means no clone. Otherwise stamps
/// the clone's table slot, links the stored cover word at `+0x18` into the
/// clone's slot at `+0x18`, and when that word is nonzero retains it through
/// the retain routine (patched thiscall callee). Returns the clone.
///
/// Original: 0x00D01440 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00d01440(this: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const INIT: u32 = 2;
        const RETAIN: u32 = 3;
        const EVENT_MANAGER_SLOT: u32 = 0x016669C8;
        const VTABLE: u32 = 0x00EDFC1C;
        const COVER: u32 = 0x18;

        let manager = lf_checker_rt::global::<u32>(EVENT_MANAGER_SLOT).read();
        let target = lf_checker_rt::callee_thiscall!(ALLOC, u32, manager);
        if target == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(INIT, u32, target);
        let cover = ((this + COVER) as *const u32).read_unaligned();
        ((target) as *mut u32).write_unaligned(VTABLE);
        ((target + COVER) as *mut u32).write_unaligned(cover);
        if cover != 0 {
            lf_checker_rt::callee_thiscall!(RETAIN, u32, cover, target + COVER);
        }
        target
    }
});
