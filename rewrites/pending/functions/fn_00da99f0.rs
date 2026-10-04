// original: 0x00da99f0 flee_raycast_forward
/// Forward a biased raycast query and store the hit marker on success.
///
/// Builds a small query block from the source point (with the third
/// coordinate biased down), forwards it with the scalar arguments to the
/// raycast routine, and on success copies the hit marker into the output
/// slot. Returns 1 on a hit, 0 on a miss. The original passes a pointer to
/// its own stack frame; the rewrite passes an equivalent local block, and
/// the contract compares the pointed-to words rather than the address.
export!(cdecl, rw_00da99f0(
    a0: u32,
    a1: u32,
    a2: f32,
    a3: u32,
    a4: u32,
    a5: f32,
    a6: u32,
    a7: u32,
    a8: u32,
) -> u32 {
    unsafe {
        /// Hit marker's offset inside the context object.
        const MARKER: u32 = 0x18;
        let f0 = f32::from_bits(((a1) as *const u32).read_unaligned());
        let f1 = f32::from_bits(((a1.wrapping_add(4)) as *const u32).read_unaligned());
        let f2 = f32::from_bits(((a1.wrapping_add(8)) as *const u32).read_unaligned()) - a2;
        // Matches the original frame block word for word: the biased
        // coordinate sits one word below the passed base, then the two plain
        // coordinates, then zero words (the original's are uninitialised
        // stack read as zero under the contract's stack fill).
        let buf = [
            f2.to_bits(),
            f0.to_bits(),
            f1.to_bits(),
            0u32,
            0,
            0,
            0,
        ];
        let r: u32 = callee_cdecl!(
            1,
            u32,
            a0,
            a1,
            (buf.as_ptr() as u32).wrapping_add(4),
            a4,
            a5.to_bits(),
            a6,
            1,
            a7,
            a8
        );
        if r as u8 == 0 {
            0
        } else {
            ((a3) as *mut u32).write_unaligned(((a4.wrapping_add(MARKER)) as *const u32).read_unaligned());
            1
        }
    }
});
