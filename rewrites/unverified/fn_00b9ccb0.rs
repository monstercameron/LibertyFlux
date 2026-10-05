// original: 0x00b9ccb0 NativeImpl_GET_SORTED_NETWORK_RESTART_NODE

/// Gets a sorted network restart node near a position.
///
/// Fills the 32-byte group table `GLOB` with 0x01 bytes, then passes the
/// input triple (`x`, `y`, `z`) plus the id out-slot (initialized to -1)
/// through the 7-argument `GET` call on the global object `OBJ` as
/// (buf, w, f, idslot, s0, s1, s2), where the last three are sort
/// parameters. When the callee's low byte is nonzero the node id plus one
/// is stored through `out` and the return is (`out` & ~0xFF) | 1;
/// otherwise zero is stored and the return is `out` & ~0xFF. A null `out`
/// faults on the store, like the original.
///
/// Both buffer pointers are skipped call arguments with snapshot-verified
/// contents (see `narrowed`); the id slot is written by the stub's
/// scripted words.
///
/// Original: 0x00B9CCB0 (cdecl, nine stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9ccb0(
    x: u32, y: u32, z: u32, w: u32, f: u32, out: u32, s0: u32, s1: u32, s2: u32,
) -> u32 {
    const OBJ: u32 = 0x01177A80;
    const GLOB: u32 = 0x01179644;
    const FILL: u32 = 0x01010101;
    const GET: u32 = 1;
    unsafe {
        let g = lf_checker_rt::relocated(GLOB) as *mut u32;
        for i in 0..8u32 {
            g.add(i as usize).write_unaligned(FILL);
        }
        let mut row = [0xFFFF_FFFFu32, x, y, z, 0];
        let base = row.as_mut_ptr();
        let ok: u32 = lf_checker_rt::callee_thiscall!(
            GET,
            u32,
            lf_checker_rt::relocated(OBJ),
            base.add(1) as u32,
            w,
            f,
            base as u32,
            s0,
            s1,
            s2
        );
        if ok & 0xFF != 0 {
            (out as *mut u32).write_unaligned(row[0].wrapping_add(1));
            (out & 0xFFFF_FF00) | 1
        } else {
            (out as *mut u32).write_unaligned(0);
            out & 0xFFFF_FF00
        }
    }
});
