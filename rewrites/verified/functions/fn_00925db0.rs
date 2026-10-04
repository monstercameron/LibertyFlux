// original: 0x00925DB0 input_lookup
/// Look up input `a1` under selector `a0`: clear, scan, then resolve.
///
/// Returns -1 while the enable byte at `0x01036780` is clear. Both indexes
/// -1 yields 0. Otherwise clears the cell for `a1`, and when `a2` has bit 2
/// set scans seven slots for `a0`, returning the 1-based position on a hit.
/// The fallback resolves `a1` through a second table, returning `a1 + 8`
/// when its entry is nonzero, else -1. Selector 0 skips straight to the
/// fallback.
export!(cdecl, rw_00925DB0(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        if *global::<u8>(0x1036780) == 0 {
            return 0xFFFF_FFFF;
        }
        let edx0 = *global::<u32>(0x1174794);
        // Shared fallback: resolve through the second table.
        let fallback = |idx: u32| -> u32 {
            if idx == 0xFFFF_FFFF {
                return 0xFFFF_FFFF;
            }
            let t = edx0.wrapping_mul(16).wrapping_add(idx).wrapping_shl(8);
            let p = relocated(0x119D1D0).wrapping_add(t) as *const u32;
            if *p == 0 {
                0xFFFF_FFFF
            } else {
                idx.wrapping_add(8)
            }
        };
        if a0 == 0xFFFF_FFFF {
            if a1 == 0xFFFF_FFFF {
                return 0;
            }
        } else if a0 == 0 {
            return fallback(a1);
        } else if a1 == 0xFFFF_FFFF {
            // No cell to clear; fall through to the scan gate.
        } else {
            let cell = relocated(0x11A0BEC).wrapping_add(a1.wrapping_shl(8)) as *mut u32;
            *cell = 0;
            // Fall through to the scan gate.
            if a2 & 4 != 0 {
                let mut p = relocated(0x119FC08).wrapping_add(edx0.wrapping_mul(0x880));
                let mut k = 1u32;
                loop {
                    if *(p as *const u32) == a0 {
                        return k;
                    }
                    k += 1;
                    p = p.wrapping_add(0x110);
                    if k >= 8 {
                        break;
                    }
                }
            }
            return fallback(a1);
        }
        // Reached when a0 == -1 (a1 != -1) or a1 == -1 with a0 not in {0,-1}:
        // clear the cell unless a1 == -1, then the scan gate.
        if a0 == 0xFFFF_FFFF {
            let cell = relocated(0x11A0BEC).wrapping_add(a1.wrapping_shl(8)) as *mut u32;
            *cell = 0;
        }
        if a2 & 4 != 0 {
            let mut p = relocated(0x119FC08).wrapping_add(edx0.wrapping_mul(0x880));
            let mut k = 1u32;
            loop {
                if *(p as *const u32) == a0 {
                    return k;
                }
                k += 1;
                p = p.wrapping_add(0x110);
                if k >= 8 {
                    break;
                }
            }
        }
        fallback(a1)
    }
});
