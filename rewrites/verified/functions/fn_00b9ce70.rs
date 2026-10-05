// original: 0x00b9ce70 NativeImpl_GET_SORTED_NETWORK_RESTART_NODE_OF_GROUP

/// Gets a sorted network restart node near a position, of one group.
///
/// Zeroes the 32-byte group table `GLOB`, sets the byte at group index
/// `g` to 1, then the same 7-argument `GET` call as `0x00B9CCB0`:
/// (buf, w, f, idslot, s0, s1, s2). Id-plus-one or zero through `out`,
/// with the same residue returns.
///
/// Both buffer pointers are skipped call arguments with snapshot-verified
/// contents (see `narrowed`); the id slot is written by the stub's
/// scripted words.
///
/// Original: 0x00B9CE70 (cdecl, ten stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9ce70(
    x: u32, y: u32, z: u32, w: u32, f: u32, out: u32, g: u32, s0: u32, s1: u32, s2: u32,
) -> u32 {
    const OBJ: u32 = 0x01177A80;
    const GLOB: u32 = 0x01179644;
    const GET: u32 = 1;
    unsafe {
        let b = lf_checker_rt::relocated(GLOB);
        let gq = b as *mut u64;
        for i in 0..4u32 {
            gq.add(i as usize).write_unaligned(0);
        }
        (b.wrapping_add(g) as *mut u8).write(1);
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
