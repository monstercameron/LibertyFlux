// original: 0x00b72d40 CTaskSimpleCarShuffle::vf17

/// Task update: shuffle the ped toward the car, then report not done.
///
/// Returns 1 at once when the vehicle at `this+0x24` is null or the abort
/// flag at `this+0x14` is set. Otherwise, unless a shuffle target is already
/// stored at `this+0x18`, runs the acquire callee (thiscall on `this`, one
/// stack word: the ped); a still-null vehicle afterwards skips the next two
/// calls (dead in the proof: the stubbed acquire cannot null it). Then runs
/// the prepare callee (thiscall on `this`, no words) and feeds the ped, the
/// vehicle, the prepare answer and `this+0x20` to the drive callee (cdecl,
/// four words), and
/// always finishes with the notify callee (thiscall on the vehicle; pushed
/// 0, target, `this+0x28`, ped). Reports not done; the low return byte is 0
/// (or 1 on the early paths) and the upper bytes repeat the notify answer
/// (the contract pins entry eax for the call-free paths).
///
/// Original: 0x00b72d40 (thiscall, one stack word: the ped).
lf_checker_rt::export!(thiscall, rw_00b72d40(this: u32, ped: u32) -> u32 {
    unsafe {
        const VEHICLE_OFF: u32 = 0x24;
        const ABORT_OFF: u32 = 0x14;
        const TARGET_OFF: u32 = 0x18;
        const DRIVE_A_OFF: u32 = 0x20;
        const EXTRA_OFF: u32 = 0x28;
        const ACQUIRE: u32 = 1;
        const PREPARE: u32 = 2;
        const DRIVE: u32 = 3;
        const NOTIFY: u32 = 4;
        let mut vehicle = ((this + VEHICLE_OFF) as *const u32).read_unaligned();
        if vehicle == 0 {
            return 1;
        }
        if ((this + ABORT_OFF) as *const u8).read() != 0 {
            return 1;
        }
        let target = ((this + TARGET_OFF) as *const u32).read_unaligned();
        if target == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(ACQUIRE, u32, this, ped);
            vehicle = ((this + VEHICLE_OFF) as *const u32).read_unaligned();
            if vehicle != 0 {
                let prep: u32 = lf_checker_rt::callee_thiscall!(PREPARE, u32, this);
                let driv = ((this + DRIVE_A_OFF) as *const u32).read_unaligned();
                // Push order is this+0x20, prep, vehicle, ped.
                let _: u32 = lf_checker_rt::callee_cdecl!(DRIVE, u32, ped, vehicle, prep, driv);
            }
        }
        let tgt = ((this + TARGET_OFF) as *const u32).read_unaligned();
        let extra = ((this + EXTRA_OFF) as *const u32).read_unaligned();
        let ans: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, vehicle, ped, extra, tgt, 0);
        ans & 0xffff_ff00
    }
});
