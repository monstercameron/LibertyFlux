// original: 0x00ccdf70 CTaskSimpleDead::vf7
/// Report the dead task's effect request to the effect manager.
///
/// When the ped argument is non-null, the ped helper runs and the effect
/// pair defaults to (0xbf, 0x2e) unless the info block at `this+0x10`
/// exists (then the pair comes from its words at `+0xc`/`+0x10`) or the
/// type helper answers 2 (then the first becomes 0xc0). The manager handle
/// (thiscall on the global pointer) returning null ends the call with 0;
/// otherwise the pair, the 1000.0 constant and the word at `this+0x14` go
/// to the effect sink (thiscall on the handle), whose answer is returned.
/// Thiscall with one stack argument.
export!(thiscall, rw_00ccdf70(this: u32, ped: u32) -> u32 {
    unsafe {
        const MGR_G: u32 = 0x0171faf4;
        const INFO_OFF: u32 = 0x10;
        const EXTRA_OFF: u32 = 0x14;
        const RATE: u32 = 0x447a0000;
        const DEF_A: u32 = 0xbf;
        const DEF_B: u32 = 0x2e;
        const ALT_A: u32 = 0xc0;
        let (mut code, mut kind) = (DEF_A, DEF_B);
        if ped != 0 {
            let _: u32 = callee_thiscall!(1, u32, ped);
            let info = (this.wrapping_add(INFO_OFF) as *const u32).read_unaligned();
            if info != 0 {
                kind = (info.wrapping_add(0x10) as *const u32).read_unaligned();
                code = (info.wrapping_add(0x0c) as *const u32).read_unaligned();
            } else {
                let t: u32 = callee_thiscall!(2, u32, ped);
                if t == 2 {
                    code = ALT_A;
                }
            }
        }
        let mgr = *global::<u32>(MGR_G);
        let h: u32 = callee_thiscall!(3, u32, mgr);
        if h == 0 {
            return 0;
        }
        let extra = (this.wrapping_add(EXTRA_OFF) as *const u32).read_unaligned();
        callee_thiscall!(4, u32, h, kind, code, RATE, extra)
    }
});
