// original: 0x0062DBA0 skyhat_set_ref_50 (proposed)

/// Replace the owned reference at `SLOT` (`+80`), then release the argument.
///
/// When the incoming reference differs from the stored one, the stored object
/// (unless null) is dropped through the drop callee (patched), the new one is
/// stored and its unsigned 16-bit count at `+0x0a` incremented. Either way the
/// argument is then released: a null argument is skipped, otherwise its count
/// is decremented and, when it reaches zero on an owned object (kind byte at
/// `+8` is 2 or 4), its slot-0 release runs with argument 1. The accumulator
/// at return holds a callee answer or an incoming leftover, so it is not
/// compared (thiscall, one argument).
lf_checker_rt::export!(thiscall, rw_0062dba0(this: u32, arg: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x50;
        const CALLEE_DROP: u32 = 1;
        let old = ((this + SLOT) as *const u32).read_unaligned();
        if arg != old {
            if old != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_DROP, u32, old);
            }
            ((this + SLOT) as *mut u32).write_unaligned(arg);
            if arg != 0 {
                let c = ((arg + 0x0a) as *const u16).read_unaligned();
                ((arg + 0x0a) as *mut u16).write_unaligned(c.wrapping_add(1));
            }
        }
        if arg != 0 {
            let count = ((arg + 0x0a) as *const u16).read_unaligned();
            if count != 0 {
                ((arg + 0x0a) as *mut u16).write_unaligned(count.wrapping_sub(1));
                let kind = ((arg + 8) as *const u8).read();
                if count == 1 && (kind == 2 || kind == 4) {
                    let rvt = (arg as *const u32).read_unaligned();
                    let rtgt = (rvt as *const u32).read_unaligned();
                    let release: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(rtgt as usize);
                    release(arg, 1);
                }
            }
        }
        0
    }
});
