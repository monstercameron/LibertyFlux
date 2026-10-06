// original: 0x00697d00 frame_setup_identity (proposed)

/// Initialise a frame matrix to identity rows, then hand it to a helper.
///
/// `mat` gets the three diagonal entries (`+0x00`, `+0x14`, `+0x28`) set to
/// 1.0 with the rest of the three 16-byte rows and the trailing row
/// (`+0x30`, `+0x34`, `+0x38`) zeroed; slots `+0x0c`, `+0x1c`, `+0x2c` and
/// `+0x3c` keep whatever they held. Then the helper is called as
/// `helper(5, 6, tag, mat, 1)` and its result is ignored.
///
/// Original: stdcall, two stack words (`tag`, `mat`), callee cleans 8.
/// Entry `ecx` is ignored.
lf_checker_rt::export!(stdcall, rw_00697d00(tag: u32, mat: u32) -> u32 {
    unsafe {
        const ONE: u32 = 0x3f800000;
        let m = mat as *mut u32;
        m.write_unaligned(ONE);
        m.add(1).write_unaligned(0);
        m.add(2).write_unaligned(0);
        m.add(4).write_unaligned(0);
        m.add(5).write_unaligned(ONE);
        m.add(6).write_unaligned(0);
        m.add(8).write_unaligned(0);
        m.add(9).write_unaligned(0);
        m.add(10).write_unaligned(ONE);
        m.add(14).write_unaligned(0);
        m.add(13).write_unaligned(0);
        m.add(12).write_unaligned(0);
        let _ = lf_checker_rt::callee_stdcall!(1, u32, 5u32, 6u32, tag, mat, 1u32);
        0
    }
});
