// original: 0x008C6680 stream_slot_publish
/// Publish streaming slot `idx` after guard checks, copying its name.
///
/// With a null name pointer only the probe callee runs. With the slot's
/// flag clear, or set but matching the caller's key through the compare
/// callee, the slot is reset, its handle resolved (bailing out on -1),
/// its name copied into the slot entry, the dispatch callee runs with the
/// scratch request, and the bind chain runs; then the flag is raised.
/// Returns the probe answer, 0, -1, or the bind answer by path.
/// Original: thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_008c6680(this: u32, name: u32,
                                              idx: u32) -> u32 {
    unsafe {
        const PROBE_CALLEE: u32 = 1;
        const CMP_CALLEE: u32 = 2;
        const RESET_CALLEE: u32 = 3;
        const HANDLE_CALLEE: u32 = 4;
        const DISPATCH_CALLEE: u32 = 5;
        const BIND_CALLEE: u32 = 6;
        const LINK_CALLEE: u32 = 7;
        const COMMIT_CALLEE: u32 = 8;
        const FINISH_CALLEE: u32 = 9;
        const COOKIE_CALLEE: u32 = 10;
        const FLAG_BASE: u32 = 0xF0;
        const ENTRY_BASE: u32 = 0x1EC;
        const ENTRY_STRIDE: u32 = 8;
        const REQ_MAGIC_FILE_VA: u32 = 0x00E7F945;
        const BIND_TOKEN_FILE_VA: u32 = 0x0110C0A0;
        if name == 0 {
            let probe: u32 =
                lf_checker_rt::callee_cdecl!(PROBE_CALLEE, u32,);
            lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return probe;
        }
        if ((this.wrapping_add(idx).wrapping_add(FLAG_BASE)) as *const u8)
            .read() != 0
        {
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
        if handle == 0xFFFF_FFFF {
            lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return 0xFFFF_FFFF;
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
        let mut scratch: u32 = 0;
        let magic = lf_checker_rt::relocated(REQ_MAGIC_FILE_VA);
        let _dispatch: u32 = lf_checker_rt::callee_thiscall!(
            DISPATCH_CALLEE, u32, this, handle,
            &mut scratch as *mut u32 as u32);
        let token = lf_checker_rt::relocated(BIND_TOKEN_FILE_VA);
        let bound: u32 = lf_checker_rt::callee_thiscall!(
            BIND_CALLEE, u32, token, _dispatch, magic, 0, 1);
        let table = ((this + 0x264) as *const u32).read_unaligned();
        let word = (table.wrapping_add(handle.wrapping_mul(16))
                    .wrapping_add(8) as *const u32)
            .read_unaligned();
        let _link_ignored: u32 = lf_checker_rt::callee_thiscall!(
            LINK_CALLEE, u32, bound, word);
        // The original has no (an instruction of the original)
        // value is discarded and esi still holds `bound` (verified byte read).
        let linked: u32 = bound;
        let _committed: u32 = lf_checker_rt::callee_thiscall!(
            COMMIT_CALLEE, u32, this, linked, idx);
        let done: u32 =
            lf_checker_rt::callee_thiscall!(FINISH_CALLEE, u32, linked);
        (this.wrapping_add(idx).wrapping_add(FLAG_BASE) as *mut u8).write(1);
        lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
        done
    }
});
