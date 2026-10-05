// original: 0x009D59C0 parse_indexed_dispatch (proposed)
//
/// Parses a string, resolves an index, and dispatches through a global table.
///
/// Parses `text` with the scanner (callee 1) into three frame slots: an id
/// (first output), a key (second) and a value (third); the out-pointers are
/// skipped and observed via snapshots. The resolver (callee 3) maps the id
/// slot to an index, or -1 (equality compare, signedness-neutral) to stop.
/// Otherwise the dword at `TABLE + index * 4` (a relocated global) is the
/// target object; a null entry stops. The key slot is converted (callee 4)
/// and the value slot plus the converted key are dispatched on the object
/// (callee 5, thiscall). The trailing security cookie check (callee 6)
/// preserves registers and is called for sequence parity. Cdecl, one word.
lf_checker_rt::export!(cdecl, rw_009D59C0(text: u32) -> u32 {
    unsafe {
        const FMT: u32 = 0xe96b40;
        const TABLE: u32 = 0x1295cd8;
        const NONE: u32 = 0xFFFFFFFF;
        const SCAN: u32 = 1;
        const SYNC: u32 = 2;
        const RESOLVE: u32 = 3;
        const CONVERT: u32 = 4;
        const DISPATCH: u32 = 5;
        const COOKIE: u32 = 6;
        let fmt = lf_checker_rt::relocated(FMT);
        let mut o_id = 0u32;
        let mut o_key = 0u32;
        let mut o_val = 0u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            SCAN, u32, text, fmt,
            (&mut o_id as *mut u32) as u32,
            (&mut o_key as *mut u32) as u32,
            (&mut o_val as *mut u32) as u32);
        let _: u32 = lf_checker_rt::callee_cdecl!(SYNC, u32,);
        let idx: u32 = lf_checker_rt::callee_cdecl!(
            RESOLVE, u32, (&mut o_id as *mut u32) as u32);
        if idx == NONE {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return idx;
        }
        let tgt: u32 = lf_checker_rt::global::<u32>(
            TABLE.wrapping_add(idx.wrapping_mul(4))).read();
        if tgt == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return idx;
        }
        let k: u32 = lf_checker_rt::callee_cdecl!(
            CONVERT, u32, (&mut o_key as *mut u32) as u32, 0);
        let r: u32 = lf_checker_rt::callee_thiscall!(
            DISPATCH, u32, tgt, (&mut o_val as *mut u32) as u32, k);
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        r
    }
});
