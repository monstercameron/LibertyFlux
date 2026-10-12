// original: 0x008A4E40 rage::audIfSound::audIfSound_2

/// Reparent an if-sound node and release its two branch children.
///
/// Writes the class vtable, then unless the static bit (`STATIC_BIT` of the
/// byte at `FLAGS`) is set releases each live branch child: child `CHILD_A`
/// with release kind 0 and child `CHILD_B` with kind 1, where a child is
/// live when its index byte is not `NONE` (0xFF) and its resolved target
/// (table slot `TABLE_BIAS + pool * ROW_BYTES` past `TABLE`, plus
/// `STRIDE * index` past `STRIDE`) is non-null. Finally tail-calls the base
/// destructor and returns its result. Thiscall, object in ECX.
lf_checker_rt::export!(thiscall, rw_008A4E40(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E7_B6A4;
        const FLAGS: u32 = 0x39;
        const STATIC_BIT: u8 = 0x40;
        const POOL: u32 = 0x40;
        const CHILD_A: u32 = 0x48;
        const CHILD_B: u32 = 0x49;
        const STRIDE: u32 = 0x0115_D964;
        const TABLE: u32 = 0x0115_D988;
        const TABLE_BIAS: u32 = 0x6F10;
        const ROW_BYTES: u32 = 0x6F40;
        const NONE: u32 = 0xFF;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if (this.wrapping_add(FLAGS) as *const u8).read_unaligned() & STATIC_BIT == 0 {
            let pool = (this.wrapping_add(POOL) as *const u8).read_unaligned() as u32;
            let stride = (lf_checker_rt::relocated(STRIDE) as *const u32).read_unaligned();
            let table = (lf_checker_rt::relocated(TABLE) as *const u32).read_unaligned();
            let child_a = (this.wrapping_add(CHILD_A) as *const u8).read_unaligned() as u32;
            if child_a != NONE {
                let slot = (table
                    .wrapping_add(TABLE_BIAS)
                    .wrapping_add(pool.wrapping_mul(ROW_BYTES))
                    as *const u32)
                    .read_unaligned();
                if slot.wrapping_add(stride.wrapping_mul(child_a)) != 0 {
                    lf_checker_rt::callee_thiscall!(1, u32, this, 0);
                }
            }
            let child_b = (this.wrapping_add(CHILD_B) as *const u8).read_unaligned() as u32;
            if child_b != NONE {
                let slot = (table
                    .wrapping_add(TABLE_BIAS)
                    .wrapping_add(pool.wrapping_mul(ROW_BYTES))
                    as *const u32)
                    .read_unaligned();
                if slot.wrapping_add(stride.wrapping_mul(child_b)) != 0 {
                    lf_checker_rt::callee_thiscall!(1, u32, this, 1);
                }
            }
        }
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
