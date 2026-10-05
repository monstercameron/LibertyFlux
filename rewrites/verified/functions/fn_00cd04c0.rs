// original: 0x00cd04c0 CTaskSimpleMeleeActionResult::vf1
/// Report the melee action result to the task manager.
///
/// Fetches the manager handle (thiscall on the global manager pointer) and,
/// when non-null, forwards four words from `this+0x14..0x1c`, the byte at
/// `this+0x20`, bit 6 of the byte at `this+0x5c`, and bits 1 and 2 of the
/// byte at `this+0x5d` to the result sink (thiscall on the handle, seven
/// arguments). Thiscall with no stack arguments.
export!(thiscall, rw_00cd04c0(this: u32) -> u32 {
    unsafe {
        const MGR_G: u32 = 0x0167e2a0;
        let mgr = *global::<u32>(MGR_G);
        let h: u32 = callee_thiscall!(1, u32, mgr);
        if h == 0 {
            return 0;
        }
        let b5d = (this.wrapping_add(0x5d) as *const u8).read();
        let b5c = (this.wrapping_add(0x5c) as *const u8).read();
        let p1 = u32::from((b5d >> 2) & 1);
        let p2 = u32::from((b5d >> 1) & 1);
        let p3 = u32::from((b5c >> 6) & 1);
        let p4 = u32::from((this.wrapping_add(0x20) as *const u8).read());
        let p5 = (this.wrapping_add(0x1c) as *const u32).read_unaligned();
        let p6 = (this.wrapping_add(0x18) as *const u32).read_unaligned();
        let p7 = (this.wrapping_add(0x14) as *const u32).read_unaligned();
        let _: u32 = callee_thiscall!(2, u32, h, p7, p6, p5, p4, p3, p2, p1);
        0
    }
});
