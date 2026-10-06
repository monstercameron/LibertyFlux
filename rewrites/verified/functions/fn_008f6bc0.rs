// original: 0x008F6BC0 Input_SwitchDevice

/// Switch the input mode register to `mode`: when it differs, the old mode
/// is unhooked through the hook slot first (unless it was -1) and the
/// secondary register is marked -1; the register is then stored and the
/// selector runs. When the mode changed, the tail slot is jumped through
/// with the fixed argument and its answer returned; otherwise the
/// selector's answer is returned. The hook and tail targets are read from
/// their global slots exactly like the original. All comparisons are
/// equality only. Convention: cdecl, one stack word.
lf_checker_rt::export!(cdecl, rw_008f6bc0(mode: u32) -> u32 {
    unsafe {
        const SELECT: u32 = 2;
        const MODE_REG: u32 = 0x010330F8;
        const MODE_REG2: u32 = 0x010330FC;
        const HOOK_ARG: u32 = 0x0117E6EC;
        const HOOK_SLOT: u32 = 0x0117E6F8;
        const TAIL_ARG: u32 = 0x0118D39C;
        const TAIL_SLOT: u32 = 0x0118D3A8;
        const OBJ: u32 = 0x0117E700;
        let cur = lf_checker_rt::global::<u32>(MODE_REG).read_unaligned();
        let mut changed = false;
        if cur != mode {
            if cur != 0xFFFF_FFFF {
                let hook: extern "cdecl" fn(u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::global::<u32>(HOOK_SLOT)
                        .read_unaligned()
                        as usize);
                hook(lf_checker_rt::relocated(HOOK_ARG));
            }
            changed = true;
            lf_checker_rt::global::<u32>(MODE_REG2)
                .write_unaligned(0xFFFF_FFFF);
        }
        lf_checker_rt::global::<u32>(MODE_REG).write_unaligned(mode);
        let r: u32 =
            lf_checker_rt::callee_thiscall!(SELECT, u32, lf_checker_rt::relocated(OBJ), mode);
        if !changed {
            return r;
        }
        let tail: extern "cdecl" fn(u32) -> u32 = core::mem::transmute(
            lf_checker_rt::global::<u32>(TAIL_SLOT).read_unaligned() as usize,
        );
        tail(lf_checker_rt::relocated(TAIL_ARG))
    }
});
