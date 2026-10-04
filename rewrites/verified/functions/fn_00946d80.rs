// original: 0x00946d80 notify_ready_slots
/// Notify every ready slot below the live count.
///
/// Walks the slot indices: each looked-up entry is tested with the readiness
/// probe, and ready entries receive the notification value. Null entries are
/// skipped. The live-count byte is re-read every iteration.
lf_checker_rt::export!(cdecl, rw_00946d80(a1: u32) -> () {
    unsafe {
        let bound = lf_checker_rt::global::<u8>(0x11d74f1);
        if *bound == 0 {
            return;
        }
        let mut bl: u8 = 0;
        loop {
            let esi = lf_checker_rt::callee_cdecl!(1, u32, bl as u32);
            if esi != 0 {
                let r = lf_checker_rt::callee_thiscall!(2, u32, esi);
                if (r & 0xff) != 0 {
                    lf_checker_rt::callee_thiscall!(3, u32, esi, a1);
                }
            }
            bl = bl.wrapping_add(1);
            if bl >= *bound {
                break;
            }
        }
    }
});
