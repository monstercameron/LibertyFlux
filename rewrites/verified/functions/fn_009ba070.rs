// original: 0x009BA070 CCamScriptInstruction_SetDrunkCam::vf2

/// Execute the SetDrunkCam script instruction: look the cam up by the
/// index at `this+0x08` (callee 1); if found and its inner object at
/// +0x110 exists, drive the drunk effect from the signed amount at
/// `this+0x10`: a negative amount runs thirteen mix calls (callee 4,
/// argument the negated amount, target stepping +0x30 per call) and
/// returns; a non-negative amount stores the operand at `this+0x0c` to
/// the drunk-level global and applies the amount (callee 5 with
/// (4, amount)).
///
/// The `[obj+0x110]==0` initialisation block below is unreachable against
/// stubbed callees (its creating callee takes no pointer arguments, so no
/// stand-in can publish the created object); the contract pins the word
/// live and the block is recorded as uncovered. No return value
/// (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009BA070(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0128_E400;
        const FIELD_INDEX: u32 = 0x08;
        const FIELD_LEVEL: u32 = 0x0c;
        const FIELD_AMOUNT: u32 = 0x10;
        const OBJ_INNER: u32 = 0x110;
        const GLOBAL_LEVEL: u32 = 0x0104_8AC0;
        const MIX_ROUNDS: u32 = 13;
        const MIX_STRIDE: u32 = 0x30;
        let index = ((this + FIELD_INDEX) as *const u32).read_unaligned();
        let cam: u32 = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(MGR), index);
        if cam == 0 {
            return 0;
        }
        let inner = ((cam + OBJ_INNER) as *const u32).read_unaligned();
        if inner == 0 {
            // Init block: unreachable in trials (pinned live). Included
            // for fidelity; its two callees are intentionally undeclared.
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, cam);
            let obj = ((cam + OBJ_INNER) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, obj);
            ((obj + 0x27c) as *mut u8).write(0);
            ((obj + 0x270) as *mut u32).write_unaligned(0);
            ((obj + 0x278) as *mut u32).write_unaligned(0);
            ((obj + 0x280) as *mut u32).write_unaligned(0xffff_ffff);
            let published = ((cam + OBJ_INNER) as *const u32).read_unaligned();
            ((published + 0x270) as *mut u32)
                .write_unaligned(cam.wrapping_add(0x10));
            let published2 = ((cam + OBJ_INNER) as *const u32).read_unaligned();
            ((published2 + 0x278) as *mut u32).write_unaligned(cam);
            return 0;
        }
        let amount = ((this + FIELD_AMOUNT) as *const i32).read_unaligned();
        if amount < 0 {
            let count = amount.wrapping_neg() as u32;
            let mut target = inner;
            for _ in 0..MIX_ROUNDS {
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(4, u32, target, count);
                target = target.wrapping_add(MIX_STRIDE);
            }
        } else {
            let level = ((this + FIELD_LEVEL) as *const u32).read_unaligned();
            lf_checker_rt::global::<u32>(GLOBAL_LEVEL).write(level);
            let w = ((this + FIELD_AMOUNT) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, inner, 4, w);
        }
        0
    }
});
