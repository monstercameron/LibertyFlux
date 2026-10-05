// original: 0x008C6B40 stream_altslot_publish
/// Publish streaming slot `idx` through the alternate table, guarded.
///
/// With a null name pointer only the probe callee runs. With the slot's
/// secondary flag set the flag itself is the result. With the primary
/// flag set but mismatching the caller's key the result is 0. Otherwise
/// the slot is reset, its handle resolved (bailing to the name copy on
/// -1), the dispatch callee runs with the scratch request, the table
/// callee binds the row words, the secondary flag is raised, and the name
/// is copied into the slot entry. Returns the probe answer, the flag, 0,
/// or the table answer by path. Original: thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_008c6b40(this: u32, name: u32,
                                              idx: u32) -> u32 {
    unsafe {
        const PROBE_CALLEE: u32 = 1;
        const FLAGB_CALLEE: u32 = 2;
        const FLAGA_CALLEE: u32 = 3;
        const CMP_CALLEE: u32 = 4;
        const RESET_CALLEE: u32 = 5;
        const HANDLE_CALLEE: u32 = 6;
        const DISPATCH_CALLEE: u32 = 7;
        const TABLE_CALLEE: u32 = 8;
        const COOKIE_CALLEE: u32 = 9;
        const FLAG_A: u32 = 0xF0;
        const FLAG_B: u32 = 0xFE;
        const ENTRY_BASE: u32 = 0x1EC;
        const ENTRY_STRIDE: u32 = 8;
        const ROW_BASE: u32 = 0x10C;
        const ROW_STRIDE: u32 = 16;
        const REQ_MAGIC_FILE_VA: u32 = 0x00E7F946;
        if name == 0 {
            let probe: u32 =
                lf_checker_rt::callee_cdecl!(PROBE_CALLEE, u32,);
            lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return probe;
        }
        let fb: u32 =
            lf_checker_rt::callee_thiscall!(FLAGB_CALLEE, u32, this, idx);
        if (fb as u8) != 0 {
            lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return fb;
        }
        let fa: u32 =
            lf_checker_rt::callee_thiscall!(FLAGA_CALLEE, u32, this, idx);
        if (fa as u8) != 0 {
            let entry = this.wrapping_add(idx.wrapping_mul(ENTRY_STRIDE))
                .wrapping_add(ENTRY_BASE);
            let diff: u32 = lf_checker_rt::callee_cdecl!(
                CMP_CALLEE, u32, entry, name);
            if diff == 0 {
                lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
                return 0;
            }
        }
        lf_checker_rt::callee_thiscall!(RESET_CALLEE, u32, this, idx);
        let handle: u32 =
            lf_checker_rt::callee_thiscall!(HANDLE_CALLEE, u32, this, name);
        // The name copy below leaves NUL in al, so the result keeps the
        // last answer's upper 24 bits with a zero low byte on every path
        // that reaches it.
        let mut last = handle;
        if handle != 0xFFFF_FFFF {
            let table = ((this + 0x264) as *const u32).read_unaligned();
            let base = table.wrapping_add(handle.wrapping_mul(16));
            let w8 = ((base + 8) as *const u32).read_unaligned();
            let w12 = ((base + 12) as *const u32).read_unaligned();
            let mut scratch: u32 = 0;
            let magic = lf_checker_rt::relocated(REQ_MAGIC_FILE_VA);
            let _dispatch: u32 = lf_checker_rt::callee_thiscall!(
                DISPATCH_CALLEE, u32, this, handle,
                &mut scratch as *mut u32 as u32);
            let row = this.wrapping_add(ROW_BASE)
                .wrapping_add(idx.wrapping_mul(ROW_STRIDE));
            let _tabled: u32 = lf_checker_rt::callee_thiscall!(
                TABLE_CALLEE, u32, row, _dispatch, w12, w8, magic);
            last = _tabled;
            ((this + idx + FLAG_B) as *mut u8).write(1);
        }
        let entry = this.wrapping_add(idx.wrapping_mul(ENTRY_STRIDE))
            .wrapping_add(ENTRY_BASE);
        let mut s = name;
        let mut d = entry;
        loop {
            let b = (s as *const u8).read();
            (d as *mut u8).write(b);
            if b == 0 {
                break;
            }
            s = s.wrapping_add(1);
            d = d.wrapping_add(1);
        }
        lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
        last & 0xFFFF_FF00
    }
});
