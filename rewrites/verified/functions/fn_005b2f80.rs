// original: 0x005B2F80 conditional_restart
/// Runs the three restart helpers when the restart flag is set and the
/// subsystem is not already restarting. Passes this object's identity with
/// its low byte forced to 1 into the first helper.
export!(thiscall, rw_005B2F80(entry: u32) -> u32 {
    unsafe {
        if *global::<u8>(0x01160C39) != 0 {
            let bits = *global::<u32>(0x018B6EA4);
            if (bits >> 9) & 1 == 0 {
                callee_thiscall!(1, u32, (entry & 0xFFFF_FF00) | 1);
                callee_cdecl!(2, u32,);
                callee_cdecl!(3, u32,);
            }
        }
        0
    }
});
