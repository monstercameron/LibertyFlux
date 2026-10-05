// original: 0x008de020 CUnLockRenderTargetDC::vf1

/// Unlock the render target held by this draw command.
///
/// Calls virtual slot `+0x40` of the device (the global at `DEVICE`) with
/// the target handle at `this + 8`, a rectangle or zero, and the constant
/// -1. When the extended flag at `this + 0x28` is set the middle argument
/// is a pointer to the rectangle at `this + 0x0c`; otherwise it is 0.
/// Thiscall, no stack arguments, one virtual outgoing call.
lf_checker_rt::export!(thiscall, rw_008de020(this: u32) -> u32 {
    unsafe {
        const DEVICE: u32 = 0x017f_5630;
        const TARGET_OFF: u32 = 8;
        const RECT_OFF: u32 = 0x0c;
        const EXTENDED_OFF: u32 = 0x28;
        const VTABLE_SLOT: u32 = 0x40;
        const FULL_LOCK: u32 = 0xffff_ffff;
        let dev = lf_checker_rt::global::<u32>(DEVICE).read_unaligned();
        let vtable = (dev as *const u32).read_unaligned();
        let unlock = ((vtable + VTABLE_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(unlock as usize);
        let target = ((this + TARGET_OFF) as *const u32).read_unaligned();
        let extended = ((this + EXTENDED_OFF) as *const u8).read();
        if extended != 0 {
            f(dev, target, this + RECT_OFF, FULL_LOCK);
        } else {
            f(dev, target, 0, FULL_LOCK);
        }
        0
    }
});
