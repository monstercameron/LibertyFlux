// original: 0x00b9c7c0 NativeImpl_GET_RANDOM_CAR_NODE_INCLUDE_SWITCHED_OFF_NODES

/// Gets a random car node near a position, including switched-off nodes.
///
/// Identical to its twin `0x00B9C730` except the seventh call argument is
/// 0 (exclude/altered filter) instead of 1: passes the input triple
/// (`x`, `y`, `z`) plus the id out-slot (initialized to -1) through the
/// 9-argument `GET` call on the global object `OBJ` as
/// (buf, w, 0, f0, f1, f2, 0, f3, idslot). When the callee's low byte is
/// nonzero the node id plus one is stored through `out` and the return is
/// (`out` & ~0xFF) | 1; otherwise zero is stored and the return is
/// `out` & ~0xFF. A null `out` faults on the store, like the original.
///
/// Both buffer pointers are skipped call arguments with snapshot-verified
/// contents (see `narrowed`); the id slot is written by the stub's
/// scripted words.
///
/// Original: 0x00B9C7C0 (cdecl, nine stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9c7c0(
    x: u32, y: u32, z: u32, w: u32, f0: u32, f1: u32, f2: u32, f3: u32, out: u32,
) -> u32 {
    const OBJ: u32 = 0x01177A80;
    const GET: u32 = 1;
    unsafe {
        let mut row = [0xFFFF_FFFFu32, x, y, z, 0];
        let base = row.as_mut_ptr();
        let ok: u32 = lf_checker_rt::callee_thiscall!(
            GET,
            u32,
            lf_checker_rt::relocated(OBJ),
            base.add(1) as u32,
            w,
            0,
            f0,
            f1,
            f2,
            0,
            f3,
            base as u32
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
