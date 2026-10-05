// original: 0x009f83d0 counted_bitset_update
use lf_k2_rt::{export, callee_cdecl, global};

/// Bits of 1.0f, pushed as the float argument to the notify calls.
const ONE_BITS: u32 = 0x3F800000;

/// Counted byte-set with realloc and notify (cdecl/3 -> eax).
///
/// When the set is live (`count != 0`) and `tag` matches the stored tag, sets
/// bit `bit` if it fits and is clear, then notifies; out-of-range or
/// already-set bits return the bit index without calling. Otherwise frees the
/// old buffer, allocates `size` bytes, zeroes the low-word many, stores the
/// tag, sets the bit and notifies. Returns the notify answer on notify paths
/// and the bit index on silent paths (both incidental but deterministic).
export!(cdecl, rw_s18f3(tag: u32, bit: u32, size: u32) -> u32 {
    unsafe {
        let tag = tag as u16;
        let bit = bit as u8;
        let count = *global::<u16>(0x12B79C4);
        if count != 0 && *global::<u16>(0x12B6288) == tag {
            if (bit as u16) >= count {
                return bit as u32;
            }
            let buf = *global::<u32>(0x12B79C0) as *mut u8;
            if *buf.add(bit as usize) != 0 {
                return bit as u32;
            }
            *buf.add(bit as usize) = 1;
        } else {
            callee_cdecl!(1, u32, *global::<u32>(0x12B79C0));
            *global::<u32>(0x12B79C0) = 0;
            *global::<u16>(0x12B79C4) = 0;
            *global::<u16>(0x12B79C6) = size as u16;
            // `(an instruction of the original); je`: a zero size skips allocation and the
            // function faults on the bit store below (not covered: the
            // contract never passes size 0).
            let buf = if size == 0 {
                0
            } else {
                callee_cdecl!(2, u32, size)
            };
            *global::<u16>(0x12B79C4) = size as u16;
            *global::<u32>(0x12B79C0) = buf;
            let n = size as u16;
            let mut k = 0u16;
            while k != n {
                *((buf.wrapping_add(k as u32)) as *mut u8) = 0;
                k = k.wrapping_add(1);
            }
            *global::<u16>(0x12B6288) = tag;
            *((buf.wrapping_add(bit as u32)) as *mut u8) = 1;
        }
        callee_cdecl!(3, u32, 0x12A, ONE_BITS)
    }
});
