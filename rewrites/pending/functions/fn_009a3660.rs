// original: 0x009a3660 station_index_picker

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

/// Read a global dword at a file VA.
#[inline(always)]
unsafe fn g_dword(file_va: u32) -> u32 {
    *global::<u32>(file_va)
}

// 0x009A3660: station-index picker (proposed name).
//
// Picks an audio station slot for the requested band id: id 1 maps to slot
// 12 and id 11 to slot 8 when the manager flag is set, id 13 draws a random
// slot, id 0 (or an empty station list) answers 0xFF, and anything else scans
// the slots for the first live match, falling back to a random pick among up
// to three collected candidates.
//
// Convention: stdcall/1, returns the slot index or 0xFF.
// Quirk: every exit runs the CRT security-cookie check; the cookie value
// itself is skipped in the contract (it folds the return-address slot, which
// legitimately differs), only the call is verified.
// ---------------------------------------------------------------------------
export!(stdcall, rw_009a3660(arg: u32) -> u32 {
    unsafe {
        let mgr = g_dword(0x01BB5624);
        callee_thiscall!(1, u32, mgr);
        let flag = *(((mgr as *const u8).add(0x169)) as *const u8);
        if flag != 0 {
            if arg == 1 {
                return scan_slots(12);
            }
            if arg == 11 {
                return scan_slots(8);
            }
        }
        if arg == 13 {
            let count = callee_cdecl!(2, u32,);
            let r = callee_cdecl!(5, u32, 0, count.wrapping_sub(1));
            callee_cdecl!(6, u32,);
            return r;
        }
        if arg == 0 {
            callee_cdecl!(6, u32,);
            return 0xFF;
        }
        scan_slots(arg)
    }
});

/// Scan the station list for `target`, returning the live match, a random
/// collected candidate, or 0xFF. The count call is repeated exactly like the
/// original (once before the loop, once per iteration check).
unsafe fn scan_slots(target: u32) -> u32 {
    let mut count = callee_cdecl!(2, u32,);
    if count == 0 {
        callee_cdecl!(6, u32,);
        return 0xFF;
    }
    let mut collect = [0u32; 3];
    let mut ncol: u32 = 0;
    let mut i: u32 = 0;
    loop {
        let e = callee_cdecl!(3, u32, i);
        if *(((e as *const u8).add(0x1904)) as *const u32) == target {
            if callee_thiscall!(4, u32, e) != 0 {
                callee_cdecl!(6, u32,);
                return i;
            }
            if ncol < 3 {
                collect[ncol as usize] = i;
                ncol += 1;
            }
        }
        i = i.wrapping_add(1);
        count = callee_cdecl!(2, u32,);
        if i >= count {
            break;
        }
    }
    if ncol == 0 {
        callee_cdecl!(6, u32,);
        return 0xFF;
    }
    let r = callee_cdecl!(5, u32, 0, ncol - 1);
    let pick = *collect.as_ptr().offset(r as isize);
    callee_cdecl!(6, u32,);
    pick
}
