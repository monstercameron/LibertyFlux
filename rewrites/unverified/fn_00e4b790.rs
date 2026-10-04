// original: 0x00e4b790 find_latest_meta_file (proposed)

/// Search every render store for the newest `.meta` file and report its
/// path: `out` receives the path with the `.meta` suffix stripped, `index`
/// receives the store number that held it.
///
/// Stores are numbered from a start value (callee 1) while a per-store
/// check (callee 2, then callee 11) accepts them. For each accepted store
/// the backend is entered (callees 3, 4), a finder object is opened over
/// the global base directory (callee 5), and its entries are walked: the
/// first entry comes from virtual slot `+0x58`, later ones from slot
/// `+0x5c` until it reports none left, and slot `+0x60` closes the walk.
/// Each entry is a record whose bytes at `+0x0` are the file path and
/// whose u64 at `+0x208` is its timestamp.
///
/// An entry counts only if the suffix check (callee 7) accepts it. The
/// accepted entry with the greatest timestamp wins (strictly greater
/// replaces the best, ties keep the earlier one): its path minus the last
/// five characters is copied to `out` (callee 8) and NUL-terminated, and
/// the store number is stored to `index`. `out[0]` is cleared first, so
/// with no accepted entry `out` stays empty and `index` is untouched.
/// Returns the backend-exit call's (callee 12) answer.
///
/// Original: 0x00e4b790 (stdcall, two stack arguments).
lf_checker_rt::export!(stdcall, rw_00e4b790(out: u32, index: u32) -> u32 {
    unsafe {
        const STORE_MGR: u32 = 0x01bb_5624;
        const VIDEOS_RENDERED: u32 = 0x00f1_84a4;
        const BASE_DIR: u32 = 0x0116_8dd8;
        const SUFFIX_META: u32 = 0x00f1_84b4;
        const FIND_DATA_LEN: usize = 0x220;
        const ENTRY_TS: usize = 0x208;
        const SUFFIX_LEN: u32 = 5;
        const VT_FIRST: u32 = 0x58;
        const VT_NEXT: u32 = 0x5c;
        const VT_CLOSE: u32 = 0x60;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn strlen(mut p: u32) -> u32 {
            unsafe {
                let mut n = 0u32;
                while rd8(p) != 0 {
                    p += 1;
                    n += 1;
                }
                n
            }
        }

        let base = lf_checker_rt::relocated(BASE_DIR);
        wr8(out, 0);
        let mgr = rd32(lf_checker_rt::relocated(STORE_MGR));
        let mut store = lf_checker_rt::callee_thiscall!(1, u32, mgr);
        let mut check = lf_checker_rt::callee_thiscall!(2, u32, mgr, store);
        if check & 0xff == 0 {
            let result = lf_checker_rt::callee_cdecl!(12, u32,);
            lf_checker_rt::callee_cdecl!(13, u32,);
            return result;
        }
        // One record buffer for the whole search, like the original's frame
        // slot: each walk's entries overwrite the previous walk's leftovers,
        // which the find-first snapshot still observes.
        let mut entry = [0u8; FIND_DATA_LEN];
        let entry_ptr = entry.as_mut_ptr() as u32;
        loop {
            lf_checker_rt::callee_cdecl!(3, u32, store);
            lf_checker_rt::callee_cdecl!(
                4,
                u32,
                lf_checker_rt::relocated(VIDEOS_RENDERED),
                0u32
            );
            let obj = lf_checker_rt::callee_cdecl!(5, u32, base, 1u32);
            let vt = rd32(obj);
            let find_first: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VT_FIRST)) as usize);
            let find_next: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VT_NEXT)) as usize);
            let find_close: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VT_CLOSE)) as usize);
            let handle = find_first(obj, base, entry_ptr);
            let mut best: u64 = 0;
            loop {
                let ok = lf_checker_rt::callee_cdecl!(
                    7,
                    u32,
                    entry_ptr,
                    lf_checker_rt::relocated(SUFFIX_META)
                );
                if ok != 0 {
                    let lo = rd32(entry_ptr + ENTRY_TS as u32);
                    let hi = rd32(entry_ptr + (ENTRY_TS + 4) as u32);
                    let ts = ((hi as u64) << 32) | lo as u64;
                    if ts > best {
                        best = ts;
                        let len = strlen(entry_ptr);
                        lf_checker_rt::callee_cdecl!(
                            8, u32, out, entry_ptr, len.wrapping_sub(SUFFIX_LEN)
                        );
                        let end = strlen(entry_ptr).wrapping_sub(SUFFIX_LEN);
                        wr8(out.wrapping_add(end), 0);
                        (index as *mut u32).write_unaligned(store);
                    }
                }
                if find_next(obj, handle, entry_ptr) & 0xff == 0 {
                    break;
                }
            }
            find_close(obj, handle);
            store = store.wrapping_add(1);
            check = lf_checker_rt::callee_thiscall!(11, u32, mgr, store);
            if check & 0xff == 0 {
                break;
            }
        }
        let result = lf_checker_rt::callee_cdecl!(12, u32,);
        lf_checker_rt::callee_cdecl!(13, u32,);
        result
    }
});
