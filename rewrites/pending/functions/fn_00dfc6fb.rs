// original: 0x00dfc6fb vsnprintf_l
/// Locale-aware bounded narrow-character formatted print.
///
/// Validates the format and buffer arguments (reporting `EINVAL` and
/// returning -1 when the format is missing, or the buffer is missing with a
/// nonzero size), formats into a caller buffer described by a small inline
/// state block, then NUL-terminates through the block, growing it when the
/// budget is exhausted. Returns the formatter's result.
export!(cdecl, rw_00dfc6fb(buf: u32, count: u32, fmt: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        if fmt == 0 || (count != 0 && buf == 0) {
            let slot = callee_cdecl!(1, u32,);
            (slot as *mut u32).write(0x16);
            callee_cdecl!(2, u32,);
            return 0xFFFF_FFFF;
        }
        let mut st = [0u32; 8];
        st[0] = buf;
        st[1] = if count > 0x7FFF_FFFF { 0x7FFF_FFFF } else { count };
        st[2] = buf;
        st[3] = 0x42;
        let r = callee_cdecl!(3, u32, st.as_ptr() as u32, fmt, a3, a4);
        if buf != 0 {
            st[1] = st[1].wrapping_sub(1);
            if (st[1] as i32) >= 0 {
                (st[0] as *mut u8).write(0);
            } else {
                callee_cdecl!(4, u32, 0, st.as_ptr() as u32);
            }
        }
        r
    }
});
