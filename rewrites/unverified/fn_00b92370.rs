// original: 0x00b92370 Text_ResolveGxtEntry

/// Resolves a text key to its entry, with an alternate formatting path.
///
/// Records a null flag byte through `flag` when non-null, then checks the
/// state block from `STATE` (called with 0): when its enable flag at
/// `FLAG_OFF` is clear, resolves `key` directly through `RESOLVE` on `OBJ`.
/// Otherwise formats into a 24-byte frame buffer through `FMT` with (`key`,
/// 24), appends the constant suffix word `SUFFIX` at the end of the string
/// starting at buffer offset 12, and resolves the buffer instead. Sets the
/// flag byte when the resolved entry is a non-empty string; a null or empty
/// result falls back to resolving `key`. Returns the entry pointer.
///
/// The buffer pointer and the cookie-check argument are uncompared frame
/// artifacts; the formatted bytes are verified through the call snapshot
/// (see `narrowed`).
///
/// Original: 0x00B92370 (cdecl, two stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b92370(key: u32, flag: u32) -> u32 {
    unsafe {
        const STATE: u32 = 1;
        const FMT: u32 = 2;
        const RESOLVE: u32 = 3;
        const COOKIE: u32 = 4;
        const OBJ: u32 = 0x0116BFF0;
        const SUFFIX: u32 = 0x00EB5394;
        const FLAG_OFF: u32 = 0x328D;
        const BUF_LEN: usize = 24;
        const STR_OFF: usize = 12;

        #[inline(always)]
        unsafe fn nonempty(p: u32) -> bool {
            unsafe { p != 0 && (p as *const u16).read_unaligned() != 0 }
        }

        if flag != 0 {
            (flag as *mut u8).write(0);
        }
        let st: u32 = lf_checker_rt::callee_cdecl!(STATE, u32, 0);
        if (st.wrapping_add(FLAG_OFF) as *const u8).read() == 0 {
            let r: u32 =
                lf_checker_rt::callee_thiscall!(RESOLVE, u32, lf_checker_rt::relocated(OBJ), key);
            let cookie = lf_checker_rt::global::<u32>(0x01057FB4).read();
            let _: u32 = lf_checker_rt::callee_thiscall!(COOKIE, u32, cookie);
            return r;
        }
        let mut buf = [0u8; BUF_LEN];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            FMT, u32, buf.as_mut_ptr() as u32, key, BUF_LEN as u32
        );
        let mut len = STR_OFF;
        while buf[len] != 0 {
            len += 1;
        }
        let suffix = lf_checker_rt::global::<u32>(SUFFIX).read();
        buf[len..len + 4].copy_from_slice(&suffix.to_le_bytes());
        let r: u32 = lf_checker_rt::callee_thiscall!(
            RESOLVE, u32, lf_checker_rt::relocated(OBJ), buf.as_mut_ptr() as u32
        );
        if flag != 0 && nonempty(r) {
            (flag as *mut u8).write(1);
        }
        let out = if nonempty(r) {
            r
        } else {
            lf_checker_rt::callee_thiscall!(RESOLVE, u32, lf_checker_rt::relocated(OBJ), key)
        };
        let cookie = lf_checker_rt::global::<u32>(0x01057FB4).read();
        let _: u32 = lf_checker_rt::callee_thiscall!(COOKIE, u32, cookie);
        out
    }
});
