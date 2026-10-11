// original: 0x00D3A7F0 CDummyTask_Wander::vf11

/// Return false in AL only when the second stack argument equals 1 and the byte at `this`+0x10 is nonzero; return true in AL for all other inputs. The callee removes three stack arguments.
lf_checker_rt::export!(thiscall, rw_00d3a7f0(this: u32, first_arg: u32, state: u32, third_arg: u32) -> u32 {
    unsafe {
        const STATE_TO_SKIP: u32 = 1;
        const ACTIVE_BYTE: u32 = 0x10;
        if state == STATE_TO_SKIP && (this.wrapping_add(ACTIVE_BYTE) as *const u8).read_unaligned() != 0 {
            0
        } else {
            1
        }
    }
});
