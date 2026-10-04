// original: 0x009539e0 advance_aux_ring
/// Append a terminator at the auxiliary cursor and rotate its ring.
///
/// Mirrors the main cursor ring on the auxiliary tables: the current entry is
/// always flagged, mode 1 returns early, mode 2 only flags, and any other
/// mode rotates (next index modulo the ring size, cursor reset, new buffer
/// selected and cleared). After a rotation the backlog drains through the
/// sampler while it stays above threshold, unless the bypass flag is set.
/// Returns 1 after a rotation, 0 on the early paths; only AL is significant.
export!(cdecl, rw_009539e0() -> u32 {
    unsafe {
        let cursor = global::<u32>(0x120F290);
        let buf = global::<u32>(0x120F294);
        *((*buf).wrapping_add(*cursor) as *mut u8) = 0;
        let mode = *global::<u8>(0x11F6FFA);
        *cursor = (*cursor).wrapping_add(1);
        if mode == 1 {
            return 0;
        }
        let cur = *global::<u8>(0x120F298);
        *(relocated(0x11F6FF0).wrapping_add(cur as u32) as *mut u8) = 1;
        if mode == 2 {
            return 0;
        }
        let size = *global::<u8>(0x11F6FFB) as u32;
        let next = ((cur as u32 + 1) % size) as u8;
        *cursor = 0;
        *global::<u8>(0x120F298) = next;
        *(relocated(0x11F6FF0).wrapping_add(next as u32) as *mut u8) = 2;
        let selected = *(relocated(0x11F6F7C).wrapping_add(next as u32 * 4) as *const u32);
        *buf = selected;
        *(selected as *mut u8) = 0;
        let again = *global::<u8>(0x11F6FFA);
        if again != 2 && again != 1 && *global::<u8>(0x1037868) == 0 {
            let mut sample: u32 = callee_cdecl!(0, u32,);
            if sample > 0x15F90 {
                loop {
                    let _: u32 = callee_cdecl!(1, u32,);
                    sample = callee_cdecl!(0, u32,);
                    if sample <= 0x15F90 {
                        break;
                    }
                }
            }
        }
        1
    }
});
