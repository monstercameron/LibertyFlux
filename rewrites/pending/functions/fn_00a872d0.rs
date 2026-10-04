// original: 0x00a872d0 notify_table_walk
/// Walk the notify table and alert on live entries.
///
/// Collects the current entry list from the enumerator (intercepted,
/// thiscall/1) into a scratch buffer, scanning it newest-first and
/// alerting (intercepted, thiscall/1) on entries whose live bit is set.
/// With a nonzero selector byte the walk runs twice and additionally
/// requires a clear secondary flag; otherwise it runs once. Returns 1;
/// only the low byte is compared.
export!(thiscall, rw_00a872d0(this_obj: u32, arg0: u32) -> u32 {
    unsafe {
        let inner = *((this_obj.wrapping_add(4)) as *const u32);
        if arg0 & 0xff != 0 {
            let mut round = 0u32;
            while round < 2 {
                let mut buf = [0u32; 60];
                let n: u32 = callee_thiscall!(1, u32, inner, buf.as_mut_ptr() as u32);
                let mut i = n.wrapping_sub(1) as i32;
                while i >= 0 {
                    let e = buf[i as usize];
                    if *((e.wrapping_add(0x13c)) as *const u8) & 2 != 0
                        && *((e.wrapping_add(0x13b)) as *const u8) == 0
                    {
                        let _: u32 = callee_thiscall!(2, u32, this_obj, e);
                    }
                    i -= 1;
                }
                round += 1;
            }
        } else {
            let mut buf = [0u32; 60];
            let n: u32 = callee_thiscall!(3, u32, inner, buf.as_mut_ptr() as u32);
            let mut i = n.wrapping_sub(1) as i32;
            while i >= 0 {
                let e = buf[i as usize];
                if *((e.wrapping_add(0x13c)) as *const u8) & 2 != 0 {
                    let _: u32 = callee_thiscall!(4, u32, this_obj, e);
                }
                i -= 1;
            }
        }
        1
    }
});
