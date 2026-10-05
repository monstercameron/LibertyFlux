// original: 0x00b9cbe0 NativeImpl_GET_SAFE_POSITION_FOR_CHAR_2

/// Gets a safe position for a character near (x, y, z), into three out-slots.
///
/// When `z` is at or below -100.0 (`Z_FLOOR`, ordered comparison, so NaN
/// skips), the height is refined by `GET_Z(x, y, 4)`, which returns an
/// f32 in ST0; the original stores it back into its own incoming `z` slot
/// (unobserved: the stack check is off, see `narrowed`) and reloads it.
/// Then `SAFE` is called with (inbuf, outbuf, 25.0, 3, flagbit, y, zef)
/// where inbuf is [x, scratch × 3], outbuf is a 3-word out-buffer,
/// flagbit is 3 when the low byte of `flag` is nonzero else 2, and zef
/// is the refined (or original) `z`. The out-triple is copied through
/// `o0`, `o1`, `o2` in order; a null out-pointer faults on its store.
/// Returns (answer & ~0xFF) | (answer != 0).
///
/// Both buffer pointers are skipped call arguments with snapshot-verified
/// contents (see `narrowed`); the out-triple is written by the stub's
/// scripted words.
///
/// Original: 0x00B9CBE0 (cdecl, seven stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9cbe0(
    x: u32, y: u32, z: u32, flag: u32, o0: u32, o1: u32, o2: u32,
) -> u32 {
    const Z_FLOOR: u32 = 0x00FE8DF8; // -100.0f in .rdata
    const GET_Z: u32 = 1;
    const SAFE: u32 = 2;
    const R: u32 = 0x41C80000; // 25.0f
    unsafe {
        let floor = f32::from_bits(
            (lf_checker_rt::relocated(Z_FLOOR) as *const u32).read_unaligned(),
        );
        let zf = f32::from_bits(z);
        let mut zn = zf;
        if floor >= zf {
            zn = lf_checker_rt::callee_cdecl!(GET_Z, f32, x, y, 4);
        }
        let fbit = if flag & 0xFF != 0 { 3u32 } else { 2u32 };
        let mut inrow = [x, 0, 0, 0];
        let mut outrow = [0u32; 3];
        let r: u32 = lf_checker_rt::callee_cdecl!(
            SAFE,
            u32,
            inrow.as_mut_ptr() as u32,
            outrow.as_mut_ptr() as u32,
            R,
            3,
            fbit,
            y,
            zn.to_bits()
        );
        (o0 as *mut u32).write_unaligned(outrow[0]);
        (o1 as *mut u32).write_unaligned(outrow[1]);
        (o2 as *mut u32).write_unaligned(outrow[2]);
        (r & 0xFFFF_FF00) | u32::from(r != 0)
    }
});
