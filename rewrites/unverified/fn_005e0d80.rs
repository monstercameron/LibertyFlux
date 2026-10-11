// original: 0x005E0D80 CTaskComplexCopHelicopter::~CTaskComplexCopHelicopter__deleting

/// Restore the class vtable, call the base task destructor, and return the
/// object pointer. When flag bit zero is set, mark this pool slot released,
/// lower the signed minimum slot index if needed, and wrapping-decrement the
/// pool's live count. The pool index is the signed quotient of the object's
/// address delta by the signed slot stride.
lf_checker_rt::export!(thiscall, rw_005e0d80(this: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00fe_0f9c;
        const TIMING_MAP_BIT: u8 = 0x80;
        const TIMING_POOL_GLOBAL: u32 = 0x0167_E2A0;
        const DESTROY_TASK_BASE: u32 = 1;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        write_u32(this, lf_checker_rt::relocated(VTABLE));
        let _ = lf_checker_rt::callee_thiscall!(DESTROY_TASK_BASE, u32, this);

        if flags & 1 != 0 {
            let timing_pool = *lf_checker_rt::global::<u32>(TIMING_POOL_GLOBAL);
            let pool_base = read_u32(timing_pool);
            let map = read_u32(timing_pool + 4);
            let stride = read_u32(timing_pool + 12) as i32;
            let pool_index = this.wrapping_sub(pool_base) as i32 / stride;
            let map_byte = map.wrapping_add(pool_index as u32);
            write_u8(map_byte, read_u8(map_byte) | TIMING_MAP_BIT);

            let minimum_index_slot = timing_pool + 16;
            if pool_index < read_u32(minimum_index_slot) as i32 {
                write_u32(minimum_index_slot, pool_index as u32);
            }
            let live_count_slot = timing_pool + 20;
            write_u32(live_count_slot, read_u32(live_count_slot).wrapping_sub(1));
        }
        this
    }
});
