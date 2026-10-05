// original: 0x009a95e0 ring_record_find_copy
/// Scan the ring for a record matching two keys and copy hits to output.
///
/// For step `s` in 1..=70 the slot is `(cursor - s + 70) mod 70`
/// (unsigned arithmetic, as the original's `div` does it) into the same
/// 70-record ring at `this+0x56c` that `ring_slot_append` writes. The
/// divisor is 70 on every iteration: the loop computes it as `s + 69`
/// before the first step, then the back edge skips that computation and
/// reloads the constant 70, so steps 2..=70 divide by 70 as well. When
/// the slot's first two words equal `key0` and `key1`, the whole
/// four-word record is copied to `out + 16 * (*count)`, and `*count`
/// is incremented. Thiscall, four stack words, no result.
export!(thiscall, rw_009A95E0(this: u32, key0: u32, key1: u32, countp: u32, out: u32) -> u32 {
    unsafe {
        const RING: u32 = 0x56c;
        const CURSOR: u32 = 0x9cc;
        const STEPS: u32 = 70;
        const STRIDE: u32 = 16;
        let rd = |off: u32| ((this + off) as *const u32).read_unaligned();
        let mut s = 1u32;
        while s <= STEPS {
            let divr = 70;
            let rem = rd(CURSOR).wrapping_sub(s).wrapping_add(70) % divr;
            let slot = this + RING + rem * STRIDE;
            let v0 = (slot as *const u32).read_unaligned();
            if v0 == key0 {
                let v1 = (slot.wrapping_add(4) as *const u32).read_unaligned();
                if v1 == key1 {
                    let n = (countp as *const u32).read_unaligned();
                    let dst = out + n * STRIDE;
                    (dst as *mut u32).write_unaligned(v0);
                    (dst.wrapping_add(4) as *mut u32).write_unaligned(v1);
                    let v2 = (slot.wrapping_add(8) as *const u32).read_unaligned();
                    (dst.wrapping_add(8) as *mut u32).write_unaligned(v2);
                    let v3 = (slot.wrapping_add(12) as *const u32).read_unaligned();
                    (dst.wrapping_add(12) as *mut u32).write_unaligned(v3);
                    (countp as *mut u32).write_unaligned(n.wrapping_add(1));
                }
            }
            s += 1;
        }
        0
    }
});
