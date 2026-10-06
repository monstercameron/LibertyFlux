// original: 0x0087c040 rage::crmtNodePair::vf1
/// Tear down a pair node: release both children, run the shared teardowns.
///
/// Releases the children at `this+0x28` and `this+0x2c` through slot 8 of
/// their own tables (each null-checked, each slot cleared afterwards), runs
/// intercepted direct callees 2 and 3 (thiscall/0) over this, zeroes `+0x0c`
/// and `+0x10`, and zeroes `+0x08` when non-zero. Returns callee 3's answer.
/// All comparisons are null checks.
///
/// Original: thiscall/0, two indirect plus two direct calls, no floats.
export!(thiscall, rw_0087c040(this: u32) -> u32 {
    /// Release slot in each child's table.
    const RELEASE_SLOT: u32 = 8;
    unsafe {
        for off in [0x28u32, 0x2Cu32] {
            let child = ((this + off) as *const u32).read_unaligned();
            if child != 0 {
                let vt = (child as *const u32).read_unaligned();
                let tgt = ((vt + RELEASE_SLOT) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                f(child);
            }
            ((this + off) as *mut u32).write_unaligned(0);
        }
        callee_thiscall!(2, u32, this);
        let r = callee_thiscall!(3, u32, this);
        ((this + 0x0C) as *mut u32).write_unaligned(0);
        ((this + 0x10) as *mut u32).write_unaligned(0);
        if ((this + 0x08) as *const u32).read_unaligned() != 0 {
            ((this + 0x08) as *mut u32).write_unaligned(0);
        }
        r
    }
});
