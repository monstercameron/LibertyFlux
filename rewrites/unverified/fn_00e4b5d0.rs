// original: 0x00e4b5d0 delete_render_files (proposed)

/// Delete the three rendered-video sidecar files (.wmv, .tag, .meta) for one
/// gallery entry, and remember the .wmv path on the entry object.
///
/// `this` points to the gallery entry: dword at `+0x1e4` is a subsystem
/// object (its dword at `+0x224` selects the storage backend), the entry's
/// file stem is the NUL-terminated string at `+0x209`, and the string at
/// `+0x228` accumulates the entry's file path.
///
/// Behaviour: notify the subsystem (callee 1, thiscall on the `+0x1e4`
/// object with a constant key) and the storage backend (callees 2 and 3),
/// then build `<base-dir><stem>` in a 0x200-byte scratch buffer, where
/// `<base-dir>` is the NUL-terminated global string. Append `.wmv` and
/// delete that file; if the deletion reports failure, take the backend
/// error code instead. Append the full `.wmv` path to the string at
/// `+0x228`. Then strip the last four characters and repeat with `.tag`
/// and with `.meta`. Returns the third round's deletion result, or the
/// error code when that deletion failed. A path that would need more than
/// the scratch buffer takes the fatal-report callee; the contract keeps
/// paths short so that never happens.
///
/// Original: 0x00e4b5d0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00e4b5d0(this: u32) -> u32 {
    unsafe {
        const SUBSYS: u32 = 0x1e4;
        const BACKEND_SLOT: u32 = 0x224;
        const RENDER_KEY: u32 = 0x00f1_79be;
        const VIDEOS_RENDERED: u32 = 0x00f1_8318;
        const BASE_DIR: u32 = 0x0116_8dd8;
        const STEM_OFF: u32 = 0x209;
        const DEST_OFF: u32 = 0x228;
        const SCRATCH_LEN: usize = 0x200;
        const DELETE_FILE_A: u32 = 0x00e7_3268;
        const GET_LAST_ERROR: u32 = 0x00e7_317c;
        const SUFFIX_WMV: &[u8] = b".wmv";
        const SUFFIX_TAG: &[u8] = b".tag";
        const SUFFIX_META: &[u8] = b".meta";

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        /// Length of a NUL-terminated string, not counting the NUL.
        #[inline(always)]
        unsafe fn strlen(mut p: u32) -> usize {
            unsafe {
                let mut n = 0usize;
                while rd8(p) != 0 {
                    p += 1;
                    n += 1;
                }
                n
            }
        }
        /// Append `n` bytes plus the NUL terminator from `src` to `dst`.
        #[inline(always)]
        unsafe fn strappend(dst: u32, src: u32, n: usize) {
            unsafe {
                core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, n + 1);
            }
        }

        let subsys = rd32(this.wrapping_add(SUBSYS));
        lf_checker_rt::callee_thiscall!(1, u32, subsys, RENDER_KEY);
        lf_checker_rt::callee_cdecl!(2, u32, rd32(subsys.wrapping_add(BACKEND_SLOT)));
        lf_checker_rt::callee_cdecl!(3, u32, 0u32, VIDEOS_RENDERED);

        let delete_file: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(rd32(lf_checker_rt::relocated(DELETE_FILE_A)) as usize);
        let last_error: extern "stdcall" fn() -> u32 =
            core::mem::transmute(rd32(lf_checker_rt::relocated(GET_LAST_ERROR)) as usize);

        let mut buf = [0u8; SCRATCH_LEN];
        let base = lf_checker_rt::relocated(BASE_DIR);
        let stem = this.wrapping_add(STEM_OFF);
        let base_len = strlen(base);
        let stem_len = strlen(stem);
        core::ptr::copy_nonoverlapping(
            base as *const u8,
            buf.as_mut_ptr(),
            base_len + 1,
        );
        strappend(
            buf.as_mut_ptr() as u32 + base_len as u32,
            stem,
            stem_len,
        );
        let mut len = base_len + stem_len;
        // One round per suffix: append it, delete, remember errors.
        let mut result = 0u32;
        for (round, suffix) in [SUFFIX_WMV, SUFFIX_TAG, SUFFIX_META]
            .into_iter()
            .enumerate()
        {
            core::ptr::copy_nonoverlapping(
                suffix.as_ptr(),
                buf.as_mut_ptr().add(len),
                suffix.len(),
            );
            len += suffix.len();
            buf[len] = 0;
            result = delete_file(buf.as_ptr() as u32);
            if result == 0 {
                result = last_error();
            }
            if round == 0 {
                // First round only: append the .wmv path to the entry.
                let dest = this.wrapping_add(DEST_OFF);
                strappend(dest + strlen(dest) as u32, buf.as_ptr() as u32, len);
            }
            // Strip the suffix for the next round.
            len -= 4;
            buf[len] = 0;
        }
        lf_checker_rt::callee_cdecl!(6, u32,);
        result
    }
});
