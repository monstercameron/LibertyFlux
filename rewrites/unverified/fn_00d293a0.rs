// original: 0x00d293a0 CPedTargetting::vf1 (symbols)

/// Acquire a target for a ped, refreshing the slot that already holds it.
///
/// Asks the target decider (intercepted) about the current-target block and
/// `arg0`; a nonzero answer ends the function. Otherwise runs the base
/// acquire (intercepted) with `(arg0, arg1)` and stops when `arg1` is zero.
/// With a live acquisition, scans the 8 slots for the one holding `arg0` and
/// for each match releases the old auxiliary handle (intercepted) and either
/// stores the candidate's handle (when its block at `arg0 + 0xb30` is
/// nonzero and its byte at `arg0 + 0x26c` has bit 2 set, retaining it first
/// with the retain helper) or clears the slot. The original leaves `eax`
/// untouched, so no return channel is compared.
///
/// Original: 0x00D293A0 (thiscall, two stack arguments).
lf_checker_rt::export!(thiscall, rw_00d293a0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const TARGET_OFF: u32 = 0x24c;
        const BLOCK_OFF: u32 = 0x224;
        const SLOT_BASE: u32 = 0x34;
        const AUX_BASE: u32 = 0x250;
        const STRIDE: u32 = 0x40;
        const COUNT: u32 = 8;
        const CAND_OFF: u32 = 0xb30;
        const CAND_FLAG_OFF: u32 = 0x26c;
        const CAND_FLAG_BIT: u8 = 4;
        let target = unsafe { ((this + TARGET_OFF) as *const u32).read_unaligned() };
        let block = unsafe { ((target + BLOCK_OFF) as *const u32).read_unaligned() };
        let decided: u32 = lf_checker_rt::callee_thiscall!(1, u32, block, arg0);
        if (decided as u8) != 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, arg0, arg1);
        if (arg1 as u8) == 0 {
            return 0;
        }
        let mut i = 0u32;
        while i < COUNT {
            if unsafe { ((this + SLOT_BASE + i * STRIDE) as *const u32).read_unaligned() } == arg0 {
                let slot = this + AUX_BASE + i * 4;
                if unsafe { (slot as *const u32).read_unaligned() } != 0 {
                    let _: u32 = lf_checker_rt::callee_stdcall!(3, u32, slot);
                }
                let cand = unsafe { ((arg0 + CAND_OFF) as *const u32).read_unaligned() };
                if cand != 0 && unsafe { ((arg0 + CAND_FLAG_OFF) as *const u8).read() } & CAND_FLAG_BIT != 0 {
                    unsafe { (slot as *mut u32).write_unaligned(cand) };
                    let _: u32 = lf_checker_rt::callee_stdcall!(4, u32, slot);
                } else {
                    unsafe { (slot as *mut u32).write_unaligned(0) };
                }
            }
            i += 1;
        }
        0
    }
});
