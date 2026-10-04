// original: 0x00946c30 teardown_slot_list
/// Release every slot in the list, compacting the array as it goes.
///
/// Runs the table pre-pass, then walks the slot array from the top down:
/// each live entry is closed and freed, and the entries above slide down one
/// place. The live count shrinks with every step and the table-empty flag is
/// set at the end. An empty list skips straight to the flag.
lf_checker_rt::export!(cdecl, rw_00946c30() -> () {
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32,);
        let cntp = lf_checker_rt::global::<u16>(0x11d76b0);
        let mut si: u16 = *cntp;
        let mut edi: i32 = (si as i32).wrapping_sub(1);
        if edi < 0 {
            *(lf_checker_rt::global::<u8>(0x11d74f1)) = 0;
            return;
        }
        loop {
            let e = lf_checker_rt::callee_cdecl!(2, u32, edi as u32);
            if e != 0 {
                lf_checker_rt::callee_thiscall!(3, u32, e);
                lf_checker_rt::callee_cdecl!(4, u32, e);
                si = *cntp;
            }
            si = *cntp;
            if edi < (si as i32).wrapping_sub(1) {
                let mut j: i32 = edi.wrapping_add(1);
                loop {
                    let arr =
                        *(lf_checker_rt::global::<u32>(0x11d76ac) as *const u32);
                    let v = *((arr.wrapping_add((j as u32).wrapping_mul(4)))
                        as *const u32);
                    *((arr.wrapping_add(((j as u32).wrapping_sub(1)).wrapping_mul(4)))
                        as *mut u32) = v;
                    si = *cntp;
                    if j >= (si as i32).wrapping_sub(1) {
                        break;
                    }
                    j = j.wrapping_add(1);
                }
            }
            si = si.wrapping_sub(1);
            edi = edi.wrapping_sub(1);
            *cntp = si;
            if edi < 0 {
                break;
            }
        }
        *(lf_checker_rt::global::<u8>(0x11d74f1)) = 0;
    }
});
