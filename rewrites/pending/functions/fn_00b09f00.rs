// original: 0x00b09f00 notify_matching_entries
/// Ask the registry for its entry list, then notify each live entry.
///
/// Collects up to sixty entries into a scratch buffer, walks the list
/// from the top down skipping slot zero, asks each entry for its status
/// through the entry's own virtual slot, and runs the shared follow-up
/// on every entry whose status reports match. Returns the last observed
/// value (the count when the list is too short to walk).
export!(cdecl, rw_00b09f00() -> u32 {
    unsafe {
        const STATUS_SLOT: usize = 0x28;
        const MATCH_CODE: u32 = 3;
        let mut buf = [0u32; 60];
        let n = callee_stdcall!(1, u32, buf.as_mut_ptr() as u32);
        let mut out = n;
        let mut i = n.wrapping_sub(1);
        while (i as i32) > 0 {
            let obj = buf[i as usize];
            let table = *(obj as *const u32) as usize;
            let at = *((table + STATUS_SLOT) as *const u32) as usize;
            let status: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(at);
            let s = status(obj);
            out = s;
            if s == MATCH_CODE {
                out = callee_thiscall!(3, u32, obj);
            }
            i = i.wrapping_sub(1);
        }
        out
    }
});
