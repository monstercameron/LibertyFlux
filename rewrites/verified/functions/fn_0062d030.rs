// original: 0x0062D030 rage::ProceduralTextureVerletWater::vf3

/// Publish one water-table index to the two consumer slots, split into thirds.
///
/// `index` (signed, at `+0x2c`) picks an entry of the u32 table at `TABLE`
/// (`+0x14`): entry `index` goes to `OUTER` (`+0x8c`) slot `+0x1c`, while the
/// quotient/remainder of `index / 3` (signed `idiv`, remainder first from
/// `index - 1`) pick entries `rem + 2` for `INNER` (`+0x84`) slots `+0x1c` and
/// `+0x20`. `VALUE` (`+0x28`) is copied to outer slot `+0x10`. Reads the table
/// and the index as signed 32-bit values. Returns the last entry loaded
/// (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0062d030(this: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x14;
        const VALUE: u32 = 0x28;
        const INDEX: u32 = 0x2c;
        const INNER: u32 = 0x84;
        const OUTER: u32 = 0x8c;
        const DIVISOR: i32 = 3;
        unsafe fn rd(base: u32, off: u32) -> u32 {
            unsafe { ((base + off) as *const u32).read_unaligned() }
        }
        unsafe fn wr(base: u32, off: u32, v: u32) {
            unsafe { ((base + off) as *mut u32).write_unaligned(v) }
        }
        let table = rd(this, TABLE);
        let index = rd(this, INDEX) as i32;
        let outer = rd(this, OUTER);
        wr(outer, 0x10, rd(this, VALUE));
        wr(outer, 0x1c, ((table + (index as u32).wrapping_mul(4)) as *const u32).read_unaligned());
        let inner = rd(this, INNER);
        let rem_lo = (index.wrapping_sub(1)).wrapping_rem(DIVISOR);
        wr(inner, 0x1c,
            ((table + (rem_lo as u32).wrapping_mul(4).wrapping_add(8)) as *const u32).read_unaligned());
        let rem_hi = index.wrapping_rem(DIVISOR);
        let last =
            ((table + (rem_hi as u32).wrapping_mul(4).wrapping_add(8)) as *const u32).read_unaligned();
        wr(inner, 0x20, last);
        last
    }
});
