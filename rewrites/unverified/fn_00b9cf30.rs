// original: 0x00b9cf30 NativeImpl_GET_SORTED_NETWORK_RESTART_NODE_USING_GROUP_LIST

/// Gets a sorted network restart node near a position, using a group list.
///
/// Copies the 32-byte group list at `GLOB_SRC` to the group table `GLOB`
/// byte by byte, then the same 7-argument `GET` call as `0x00B9CCB0`:
/// (buf, w, f, idslot, s0, s1, s2). Id-plus-one or zero through `out`,
/// with the same residue returns.
///
/// Both buffer pointers are skipped call arguments with snapshot-verified
/// contents (see `narrowed`); the id slot is written by the stub's
/// scripted words.
///
/// Original: 0x00B9CF30 (cdecl, nine stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9cf30(
    x: u32, y: u32, z: u32, w: u32, f: u32, out: u32, s0: u32, s1: u32, s2: u32,
) -> u32 {
    const OBJ: u32 = 0x01177A80;
    const GLOB: u32 = 0x01179644;
    const GLOB_SRC: u32 = 0x01179664;
    const GET: u32 = 1;
    unsafe {
        let d = lf_checker_rt::relocated(GLOB) as *mut u8;
        let s = lf_checker_rt::relocated(GLOB_SRC) as *const u8;
        for i in 0..32u32 {
            d.add(i as usize).write(s.add(i as usize).read());
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
