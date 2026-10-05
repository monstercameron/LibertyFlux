// original: 0x009BA1C0 CCamScriptInstruction_SetFollowVehicleCamOffset::vf2

/// Execute the SetFollowVehicleCamOffset script instruction: copy four
/// 32-bit offset components from `this+0x10`..`this+0x1c` into four
/// consecutive engine globals, and the flag byte at `this+0x08` into a
/// fifth global's low byte.
///
/// All moves are bit-copies through SSE registers (`movss`) or a byte move;
/// no arithmetic is performed. No calls, no return value (thiscall).
lf_checker_rt::export!(thiscall, rw_009BA1C0(this: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x08;
        const COMP0: u32 = 0x10;
        const GLOBAL_VEC: u32 = 0x012D_D270;
        const GLOBAL_FLAG: u32 = 0x012B_D190;
        let dst = lf_checker_rt::global::<u32>(GLOBAL_VEC);
        for i in 0..4u32 {
            let bits = ((this + COMP0 + i * 4) as *const u32).read_unaligned();
            dst.add(i as usize).write(bits);
        }
        let flag = ((this + FLAG) as *const u8).read();
        lf_checker_rt::global::<u8>(GLOBAL_FLAG).write(flag);
        0
    }
});
