// original: 0x005E1160 CPedDataCop::~CPedDataCop__deleting

/// Stamp the derived vtable, detach a non-null timing-node field, and call
/// the local cleanup helper while the derived vtable is still installed.
/// Then stamp the base vtable and return `this`. When flag bit zero is set,
/// compute the signed pool index from the object address and signed stride,
/// set the timing-map marker, lower the signed minimum index if needed, and
/// wrapping-decrement the live count.
lf_checker_rt::export!(thiscall, rw_005e1160(this: u32, flags: u32) -> u32 {
    unsafe {
        const TIMING_NODE: u32 = 0x18;
        const DERIVED_VTABLE: u32 = 0x00fe_0ff4;
        const BASE_VTABLE: u32 = 0x00fe_0aa4;
        const TRACKING_POOL_GLOBAL: u32 = 0x018b_6f20;
        const TIMING_MAP_BIT: u8 = 0x80;
        const DETACH_TIMING_NODE: u32 = 1;
        const CLEANUP_COP_DATA: u32 = 2;

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

        write_u32(this, lf_checker_rt::relocated(DERIVED_VTABLE));
        let timing_node_slot = this.wrapping_add(TIMING_NODE);
        let timing_node = read_u32(timing_node_slot);
        if timing_node != 0 {
            let _ = lf_checker_rt::callee_thiscall!(
                DETACH_TIMING_NODE,
                u32,
                timing_node,
                timing_node_slot
            );
        }

        let _ = lf_checker_rt::callee_thiscall!(CLEANUP_COP_DATA, u32, this);
        write_u32(this, lf_checker_rt::relocated(BASE_VTABLE));

        if flags & 1 != 0 {
            let tracking_pool = *lf_checker_rt::global::<u32>(TRACKING_POOL_GLOBAL);
            let pool_base = read_u32(tracking_pool);
            let map = read_u32(tracking_pool + 4);
            let stride = read_u32(tracking_pool + 12) as i32;
            let pool_index = this.wrapping_sub(pool_base) as i32 / stride;
            let map_byte = map.wrapping_add(pool_index as u32);
            write_u8(map_byte, read_u8(map_byte) | TIMING_MAP_BIT);

            let minimum_index_slot = tracking_pool + 16;
            if pool_index < read_u32(minimum_index_slot) as i32 {
                write_u32(minimum_index_slot, pool_index as u32);
            }
            let live_count_slot = tracking_pool + 20;
            write_u32(live_count_slot, read_u32(live_count_slot).wrapping_sub(1));
        }
        this
    }
});
