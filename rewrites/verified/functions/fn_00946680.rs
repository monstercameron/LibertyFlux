// original: 0x00946680 teardown_file_slots
/// Tear down the file-slot table: pre-pass, handle releases, slot drain.
///
/// Runs the pre-pass helper, releases two stored handles (calling the
/// releaser with the old value, then clearing the slot), then drains every
/// slot below the live count, handing each looked-up entry to the slot
/// closer. The count byte is re-read every iteration.
lf_checker_rt::export!(cdecl, rw_00946680() -> () {
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32,);
        let g1 = lf_checker_rt::global::<u32>(0x11d74fc);
        if *g1 != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, *g1, 0);
            *g1 = 0;
        }
        let g2 = lf_checker_rt::global::<u32>(0x11d7500);
        if *g2 != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, *g2, 0);
            *g2 = 0;
        }
        let bound = lf_checker_rt::global::<u8>(0x11d74f1);
        let mut esi: u32 = 0;
        loop {
            if esi >= (*bound as u32) {
                break;
            }
            let e = lf_checker_rt::callee_cdecl!(3, u32, esi);
            lf_checker_rt::callee_thiscall!(4, u32, e);
            esi = esi.wrapping_add(1);
        }
    }
});
