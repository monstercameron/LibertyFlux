// original: 0x0087c1b0 crmt_blend_node_attach (proposed)
/// Attach pass over a blend node, fed by two incoming vector lanes.
///
/// Feeds the incoming XMM1 low lane to slot 0x38 of this node's table,
/// stores the incoming XMM2 low lane at `+0x24` and the low byte of arg0 at
/// `+0x30`, then for each of (arg1, `+0x28`) and (arg2, `+0x2c`): add-refs
/// the new argument through slot 4 of its table when non-null, releases the
/// old child through slot 8 of its table when non-null, and stores the new
/// argument. Incoming XMM1/XMM2 arrive through the checker's vector-register
/// transport. Computes no return value. All comparisons are null checks.
///
/// Original: thiscall/3, five indirect calls, vector entry values, no x87.
export!(thiscall, rw_0087c1b0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    /// Pass slot in this node's table (takes the XMM1 lane).
    const PASS_SLOT: u32 = 0x38;
    /// Add-ref slot in an argument's table.
    const ADDREF_SLOT: u32 = 4;
    /// Release slot in an old child's table.
    const RELEASE_SLOT: u32 = 8;
    /// Weight lane, written from XMM2.
    const WEIGHT_OFF: u32 = 0x24;
    /// Flag byte, written from arg0's low byte.
    const FLAG_OFF: u32 = 0x30;
    unsafe {
        let x1 = xmm_word(1, 0);
        let x2 = xmm_word(2, 0);
        let vt = (this as *const u32).read_unaligned();
        let tgt = ((vt + PASS_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        f(this, x1);
        ((this + WEIGHT_OFF) as *mut u32).write_unaligned(x2);
        ((this + FLAG_OFF) as *mut u8).write(a0 as u8);
        for (off, arg) in [(0x28u32, a1), (0x2Cu32, a2)] {
            if arg != 0 {
                let va = (arg as *const u32).read_unaligned();
                let ta = ((va + ADDREF_SLOT) as *const u32).read_unaligned();
                let fa: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(ta as usize);
                fa(arg);
            }
            let old = ((this + off) as *const u32).read_unaligned();
            if old != 0 {
                let vo = (old as *const u32).read_unaligned();
                let to = ((vo + RELEASE_SLOT) as *const u32).read_unaligned();
                let fo: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(to as usize);
                fo(old);
            }
            ((this + off) as *mut u32).write_unaligned(arg);
        }
        0
    }
});
