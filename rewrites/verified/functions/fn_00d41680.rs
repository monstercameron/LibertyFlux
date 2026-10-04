// original: 0x00d41680 CTaskSimpleJumpInAir::vf17

/// Advance a jump-in-air task's landing state machine (slot vf17).
///
/// `this` is the task, `owner` (arg0) the ped. Bit 13 of the owner word
/// at `OWNER_MARK` (+0x29c) is always set first. The state word at
/// `STATE` (+0x14) then drives a loop: -1 returns 0; 2 clears bit 13 of
/// the owner word at `OWNER_CLR` (+0x26c) and returns 1; 1 runs the
/// notify callee (thiscall on `this` with `owner`); 0 sets bit 13 of
/// `OWNER_CLR`, snapshots the tick global at `TICK` into `TICK_SAVE`
/// (+0x44), and unless bit 2 of `TASK_FLAGS` (+0x70) is set, runs the
/// trace callee (cdecl on `owner` and `SPEED_PTR`, the owner's +0x20
/// block plus 0x30): a nonzero trace stores state 2, otherwise the aim
/// callee runs (thiscall on `this`: `owner`, the dwords at +0x18/+0x1c
/// and the float at +0x20) and its zero/nonzero stores state 2/1; any
/// other state value does nothing. After each pass the state is compared
/// with its entry value: changed it loops, unchanged it returns 0. Only
/// al carries the result.
///
/// Original: 0x00d41680 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d41680(this: u32, owner: u32) -> u32 {
    unsafe {
        const OWNER_MARK: u32 = 0x29c;
        const OWNER_CLR: u32 = 0x26c;
        const STATE: u32 = 0x14;
        const TASK_FLAGS: u32 = 0x70;
        const TICK: u32 = 0x0117_35b4;
        const TICK_SAVE: u32 = 0x44;
        const NOTIFY: u32 = 1;
        const TRACE: u32 = 2;
        const AIM: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mark = (owner + OWNER_MARK) as *mut u32;
        mark.write_unaligned(mark.read_unaligned() | 0x2000);
        if rd32(this + STATE) == 0xffff_ffff {
            return 0;
        }
        loop {
            let entry = rd32(this + STATE);
            if entry == 0 {
                let clr = (owner + OWNER_CLR) as *mut u32;
                clr.write_unaligned(clr.read_unaligned() | 0x2000);
                let tick = lf_checker_rt::global::<u32>(TICK).read_unaligned();
                ((this + TICK_SAVE) as *mut u32).write_unaligned(tick);
                let skip_trace = ((this + TASK_FLAGS) as *const u8).read() & 4 != 0;
                let traced = if skip_trace {
                    0
                } else {
                    let speed = rd32(owner + 0x20);
                    lf_checker_rt::callee_cdecl!(TRACE, u32, owner, speed.wrapping_add(0x30))
                };
                if !skip_trace && traced & 0xff != 0 {
                    ((this + STATE) as *mut u32).write_unaligned(2);
                } else {
                    let aimed: u32 = lf_checker_rt::callee_thiscall!(
                        AIM, u32, this, owner,
                        rd32(this + 0x18), rd32(this + 0x1c), rd32(this + 0x20)
                    );
                    ((this + STATE) as *mut u32)
                        .write_unaligned(if aimed & 0xff != 0 { 1 } else { 2 });
                }
            } else if entry == 1 {
                lf_checker_rt::callee_thiscall!(NOTIFY, u32, this, owner);
            } else if entry == 2 {
                let clr = (owner + OWNER_CLR) as *mut u32;
                clr.write_unaligned(clr.read_unaligned() & 0xffff_dfff);
                return 1;
            }
            if rd32(this + STATE) != entry {
                continue;
            }
            return 0;
        }
    }
});
