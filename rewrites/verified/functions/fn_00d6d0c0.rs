// original: 0x00d6d0c0 replay_bar_select_entry
/// Select entry `idx` of the bar's table and publish its stamp when new.
///
/// Records `idx` at +0xA0/+0xF8 with a -1 marker at +0xFC (after notifying
/// callee 1 when the selection changed), then compares the entry stamp at
/// +0x14 against two samples of callee 2 and, on each strict difference,
/// notifies callee 3 and publishes the stamp to 0x11F70E4 with a flag at
/// 0x11F70E0. Returns the published stamp when either notify path ran, else
/// the second callee-2 sample, or the table pointer when the table is empty.
lf_checker_rt::export!(thiscall, rw_00d6d0c0(this_ptr: u32, idx: u32) -> u32 {
    unsafe {
        let b = this_ptr as *const u8;
        let table = *((b.add(0x9c)) as *const u32);
        let count = *(((table as *const u8).add(4)) as *const u16);
        if count == 0 {
            return table;
        }
        let cur = *((b.add(0xf8)) as *const u32);
        if cur != idx {
            lf_checker_rt::callee_thiscall!(1, u32, this_ptr);
        }
        *((b.add(0xa0)) as *mut u32) = idx;
        *((b.add(0xf8)) as *mut u32) = idx;
        *((b.add(0xfc)) as *mut u32) = 0xFFFF_FFFF;
        let arr = *((table as *const u8) as *const u32);
        let elem = *(((arr as *const u8).add(idx.wrapping_mul(4) as usize)) as *const u32);
        let stamp = *(((elem as *const u8).add(0x14)) as *const u32);
        let v1: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        let above = stamp > v1;
        let v2: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        let below = stamp < v2;
        let mut ret = v2;
        if above {
            lf_checker_rt::callee_cdecl!(3, u32, 6);
            *lf_checker_rt::global::<u32>(0x11f70e0) |= 1;
            *lf_checker_rt::global::<u32>(0x11f70e4) = stamp;
            ret = stamp;
        }
        if below {
            lf_checker_rt::callee_cdecl!(3, u32, 0x0e);
            *lf_checker_rt::global::<u32>(0x11f70e0) |= 1;
            *lf_checker_rt::global::<u32>(0x11f70e4) = stamp;
            ret = stamp;
        }
        ret
    }
});
