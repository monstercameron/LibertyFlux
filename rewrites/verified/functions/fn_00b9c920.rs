// original: 0x00b9c920 NativeImpl_GET_RANDOM_NETWORK_RESTART_NODE_EXCLUDING_GROUP

/// Gets a random network restart node near a position, excluding a group.
///
/// Fills the 32-byte group table `GLOB` with 0x01 bytes, clears the byte
/// at group index `g` (a wild index faults or hits adjacent globals
/// exactly like the original), then the same 7-argument `GET` call as its
/// twin `0x00B9C850`: (buf, w, f, idslot, 0, 0, 80.0). Id-plus-one or
/// zero through `out`, with the same residue returns.
///
/// Both buffer pointers are skipped call arguments with snapshot-verified
/// contents (see `narrowed`); the id slot is written by the stub's
/// scripted words.
///
/// Original: 0x00B9C920 (cdecl, seven stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9c920(
    x: u32, y: u32, z: u32, w: u32, f: u32, out: u32, g: u32,
) -> u32 {
    const OBJ: u32 = 0x01177A80;
    const GLOB: u32 = 0x01179644;
    const FILL: u32 = 0x01010101;
    const RADIUS: u32 = 0x42A00000; // 80.0f
    const GET: u32 = 1;
    unsafe {
        let b = lf_checker_rt::relocated(GLOB);
        let gw = b as *mut u32;
        for i in 0..8u32 {
            gw.add(i as usize).write_unaligned(FILL);
        }
        (b.wrapping_add(g) as *mut u8).write(0);
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
            0,
            0,
            RADIUS
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
