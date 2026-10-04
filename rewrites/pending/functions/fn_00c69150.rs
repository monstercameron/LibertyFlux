// original: 0x00c69150 pending_request_check
// Test whether a request is still pending. A fast token pre-check can
// answer 1 at once; otherwise each of the counted entries in the embedded
// array at +0x404 whose object has bit 5 of the word at +0x124 set is
// re-polled, and a negative poll answers 1. Returns 0 when nothing pends.
export!(thiscall, rw_00c69150(obj: u32) -> u32 {
    unsafe {
        let token = *global::<u32>(0x12b4138);
        let cur = *global::<u32>(0x12fa644);
        if callee_cdecl!(1, u32, cur, token) & 0xff != 0 {
            if callee_cdecl!(2, u32, cur, token) & 0xff == 0 {
                return 1;
            }
        }
        let arr = obj.wrapping_add(0x404);
        let count: u32 = callee_thiscall!(3, u32, arr);
        let n = count as i32;
        let mut s = 0i32;
        if n <= 0 {
            return 0;
        }
        let tab = relocated(0x1295cd8);
        while s < n {
            let wid: u32 = callee_thiscall!(4, u32, arr, s as u32);
            let ent = *(tab.wrapping_add(wid.wrapping_mul(4)) as *const u32);
            let f = *((ent as *const u8).add(0x124) as *const u32);
            if f & 0x20 != 0 {
                if callee_cdecl!(5, u32, wid, token) & 0xff == 0 {
                    return 1;
                }
            }
            s += 1;
        }
        0
    }
});
