// original: 0x00e17030 qt_build_ext_path (proposed)

/// Build the QuickTime extension path into `dst` and return its length.
///
/// `dst` receives the result; `src` is a second path the lookup helpers use
/// (handed on, less ten bytes, as their context argument). Two 260-byte
/// scratch buffers live in the frame: a system-directory buffer and an
/// extension-directory buffer.
///
/// Behaviour: a lookup callee is first asked to fill `dst` (given two
/// read-only registry strings and both arguments); if `dst` is non-empty
/// afterwards its length is the result. Otherwise the system directory is
/// fetched through a helper pointer kept in read-only data and the extension
/// directory is fetched by a second callee; when the extension directory is
/// empty `dst` stays empty. The system directory gains a
/// trailing separator (a two-byte suffix from read-only data) unless it
/// already ends in a backslash or a slash, then the extension directory is
/// copied over `dst`. Only when both buffers agree byte for byte is the
/// eleven-byte suffix from read-only data appended to `dst`.
///
/// The result is always the length of the NUL-terminated string in `dst`.
/// Comparison is unsigned byte-wise. The contract guarantees the
/// system-directory buffer is never empty at the separator step: the
/// original would read one byte below an empty buffer (a saved-register byte
/// the rewrite cannot observe), so that path is excluded from the proof.
///
/// Original: 0x00e17030 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00e17030(dst: u32, src: u32) -> u32 {
    unsafe {
        const REG_KEY: u32 = 0x00f13b84;
        const REG_VALUE: u32 = 0x00f13bc4;
        const SYSDIR_SLOT: u32 = 0x00e73094;
        const SEPARATOR_SUFFIX: u32 = 0x00f13af4;
        const EXT_SUFFIX: u32 = 0x00f13bb8;
        const PATH_LEN: usize = 260;
        const BACKSLASH: u8 = 0x5c;
        const SLASH: u8 = 0x2f;
        const LOOKUP_CALLEE: u32 = 1;
        const EXTDIR_CALLEE: u32 = 2;

        unsafe fn strlen(p: u32) -> u32 {
            unsafe {
                let mut n = 0u32;
                while (p as *const u8).add(n as usize).read() != 0 {
                    n += 1;
                }
                n
            }
        }

        unsafe fn strcpy(d: u32, s: u32) {
            unsafe {
                let mut i = 0usize;
                loop {
                    let c = (s as *const u8).add(i).read();
                    (d as *mut u8).add(i).write(c);
                    if c == 0 {
                        return;
                    }
                    i += 1;
                }
            }
        }

        unsafe fn strcmp(a: u32, b: u32) -> i32 {
            unsafe {
                let mut i = 0usize;
                loop {
                    let x = (a as *const u8).add(i).read();
                    let y = (b as *const u8).add(i).read();
                    if x != y {
                        return if x < y { -1 } else { 1 };
                    }
                    if x == 0 {
                        return 0;
                    }
                    i += 1;
                }
            }
        }

        lf_checker_rt::callee_cdecl!(
            LOOKUP_CALLEE,
            u32,
            lf_checker_rt::relocated(REG_KEY),
            lf_checker_rt::relocated(REG_VALUE),
            dst,
            src
        );
        if (dst as *const u8).read() != 0 {
            return strlen(dst);
        }
        let base = src.wrapping_sub(10);
        let mut sysdir = [0u8; PATH_LEN];
        let mut extdir = [0u8; PATH_LEN];
        let fetch = lf_checker_rt::global::<u32>(SYSDIR_SLOT).read();
        let get_sysdir: extern "stdcall" fn(u32, u32) -> u32 =
            core::mem::transmute(fetch as usize);
        get_sysdir(sysdir.as_mut_ptr() as u32, base);
        lf_checker_rt::callee_cdecl!(
            EXTDIR_CALLEE,
            u32,
            extdir.as_mut_ptr() as u32,
            base,
            0
        );
        if extdir[0] == 0 {
            return strlen(dst);
        }
        let len = strlen(sysdir.as_ptr() as u32) as usize;
        let last = sysdir[len - 1];
        if last != BACKSLASH && last != SLASH {
            let suffix = lf_checker_rt::global::<u16>(SEPARATOR_SUFFIX).read_unaligned();
            (sysdir.as_mut_ptr().add(len) as *mut u16).write_unaligned(suffix);
        }
        strcpy(dst, extdir.as_ptr() as u32);
        if strcmp(sysdir.as_ptr() as u32, extdir.as_ptr() as u32) != 0 {
            return strlen(dst);
        }
        // Eleven suffix bytes in the original's access widths: word, word,
        // half-word, byte.
        let dlen = strlen(dst) as usize;
        let tail = (dst as *mut u8).add(dlen);
        let word0 = lf_checker_rt::global::<u32>(EXT_SUFFIX).read_unaligned();
        let word1 = lf_checker_rt::global::<u32>(EXT_SUFFIX + 4).read_unaligned();
        let word2 = lf_checker_rt::global::<u16>(EXT_SUFFIX + 8).read_unaligned();
        let byte3 = lf_checker_rt::global::<u8>(EXT_SUFFIX + 10).read();
        (tail as *mut u32).write_unaligned(word0);
        (tail.add(4) as *mut u32).write_unaligned(word1);
        (tail.add(8) as *mut u16).write_unaligned(word2);
        tail.add(10).write(byte3);
        strlen(dst)
    }
});
