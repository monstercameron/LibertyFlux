// original: 0x00ab9fc0 format_indexed_byte_shifted
/// Format the table entry with the byte `a1` and its shifted form.
///
/// Same shape as `rw_00ab9b70` with a five-argument formatter call:
/// `(frame_buf, format, table[a0], a1 as byte, (a2 as byte) + 0x61)`,
/// then the sink and the stack-cookie check. Note the last two bytes
/// come from different argument slots: the pre-push read hits the third
/// slot while the post-push read hits the second. Returns nothing
/// meaningful.
lf_checker_rt::export!(cdecl, rw_00ab9fc0(a0: u32, a1: u32, a2: u32) -> u32 {
    let mut frame = [0u32; 21];
    let base = frame.as_mut_ptr() as u32;
    // SAFETY: as in rw_00ab9b70.
    let t = unsafe { lf_checker_rt::global::<u32>(0x0103_EE8C).offset(a0 as isize).read_unaligned() };
    let b = a1 & 0xFF;
    let b2 = a2 & 0xFF;
    let dst1 = base.wrapping_add(8);
    let _ = lf_checker_rt::callee_cdecl!(
        1, u32, dst1, lf_checker_rt::relocated(0x00EA_55FC), t, b, b2.wrapping_add(0x61));
    let dst2 = base.wrapping_add(16);
    let _ = lf_checker_rt::callee_cdecl!(2, u32, dst2);
    lf_checker_rt::callee_cdecl!(3, u32,)
});

