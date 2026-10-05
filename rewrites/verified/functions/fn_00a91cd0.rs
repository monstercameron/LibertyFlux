// original: 0x00a91cd0 stream_hash_range_checked (proposed)

/// Hash a looked-up address range, plus two global salts.
///
/// The argument is resolved (callee 1, cdecl/2 with the argument and 0x5c);
/// a null answer returns -1. Its end is resolved (callee 2, cdecl/2 with
/// the start and 0x2e); a null end returns -1 too. Otherwise the bytes from
/// one past the start to the end are copied to a stack buffer, the two
/// global salt dwords are appended, and the buffer is hashed (callee 3,
/// cdecl/1).
///
/// Returns the hash, or -1 on either miss. Cdecl, one argument. (The
/// stack-cookie check is mirrored as a no-op call so the call logs match.)
lf_checker_rt::export!(cdecl, rw_00a91cd0(arg: u32) -> u32 {
    unsafe {
        const MISS: u32 = 0xffffffff;
        const SALT1: u32 = 0x00ea3110;
        const SALT2: u32 = 0x00ea3114;
        const RESOLVE_ARG: u32 = 0x5c;
        const END_ARG: u32 = 0x2e;
        let start = lf_checker_rt::callee_cdecl!(1, u32, arg, RESOLVE_ARG);
        if start == 0 {
            lf_checker_rt::callee_cdecl!(4, u32,);
            return MISS;
        }
        let end = lf_checker_rt::callee_cdecl!(2, u32, start, END_ARG);
        if end == 0 {
            lf_checker_rt::callee_cdecl!(4, u32,);
            return MISS;
        }
        let mut buf = [0u8; 64];
        let mut i = 0usize;
        let mut p = start.wrapping_add(1);
        while p != end {
            buf[i] = (p as *const u8).read();
            i += 1;
            p = p.wrapping_add(1);
        }
        let s1 = lf_checker_rt::global::<u32>(SALT1).read_unaligned();
        let s2 = lf_checker_rt::global::<u32>(SALT2).read_unaligned();
        buf[i..i + 4].copy_from_slice(&s1.to_le_bytes());
        buf[i + 4..i + 8].copy_from_slice(&s2.to_le_bytes());
        let h = lf_checker_rt::callee_cdecl!(3, u32, buf.as_ptr() as u32);
        lf_checker_rt::callee_cdecl!(4, u32,);
        h
    }
});
