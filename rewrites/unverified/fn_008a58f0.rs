// original: 0x008A58F0 rage::audForLoopSound::~audForLoopSound__deleting

/// Destroy a for-loop sound, freeing it when asked.
///
/// Writes the class vtable, releases the loop child when its index byte
/// `CHILD` is not `NONE` (0xFF) and its resolved target (table slot
/// `TABLE_BIAS + pool * ROW_BYTES` past `TABLE`, plus `STRIDE * index`)
/// is non-null, runs the base destructor, then when bit 0 of `flags` is
/// set releases the object through the audio heap with its pool byte.
/// Returns the object pointer. Thiscall, object in ECX, flags popped from
/// the stack.
lf_checker_rt::export!(thiscall, rw_008A58F0(this: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E7_B80C;
        const POOL: u32 = 0x40;
        const CHILD: u32 = 0x48;
        const STRIDE: u32 = 0x0115_D964;
        const TABLE: u32 = 0x0115_D988;
        const TABLE_BIAS: u32 = 0x6F10;
        const ROW_BYTES: u32 = 0x6F40;
        const HEAP: u32 = 0x0115_D8A0;
        const DELETE: u32 = 1;
        const NONE: u32 = 0xFF;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let child = (this.wrapping_add(CHILD) as *const u8).read_unaligned() as u32;
        if child != NONE {
            let pool = (this.wrapping_add(POOL) as *const u8).read_unaligned() as u32;
            let stride = (lf_checker_rt::relocated(STRIDE) as *const u32).read_unaligned();
            let table = (lf_checker_rt::relocated(TABLE) as *const u32).read_unaligned();
            let slot = (table
                .wrapping_add(TABLE_BIAS)
                .wrapping_add(pool.wrapping_mul(ROW_BYTES))
                as *const u32)
                .read_unaligned();
            if slot.wrapping_add(stride.wrapping_mul(child)) != 0 {
                lf_checker_rt::callee_thiscall!(1, u32, this, 0);
            }
        }
        lf_checker_rt::callee_thiscall!(2, u32, this);
        if flags & DELETE != 0 {
            let pool = (this.wrapping_add(POOL) as *const u8).read_unaligned();
            lf_checker_rt::callee_thiscall!(
                3,
                u32,
                lf_checker_rt::relocated(HEAP),
                this,
                u32::from(pool)
            );
        }
        this
    }
});
