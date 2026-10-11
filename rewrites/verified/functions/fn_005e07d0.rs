// original: 0x005E07D0 CTaskComplexDriveFireTruck::~CTaskComplexDriveFireTruck__deleting

/// Stamp the task's vtable, detach both timing-node references before running the `CTaskComplex`
/// destructor. The flags argument is popped by the thiscall routine; bit 0
/// controls a second release from the global timing pool. That path uses a
/// signed quotient of the object's address delta and the pool stride to set
/// a high bit in the pool's byte map, lowers the signed minimum index when
/// needed, and decrements the pool's live count with wrapping arithmetic.
/// The destructor returns the original object pointer. Each timing-node
/// detach receives the field's current pointer in ECX and the field address
/// as its single stack argument.
lf_checker_rt::export!(thiscall, rw_005e07d0(this: u32, flags: u32) -> u32 {
    unsafe {
        const TIMING_NODE_A: u32 = 0x14;
        const TIMING_NODE_B: u32 = 0x1c;
        const TIMING_MAP: u8 = 0x80;
        const VTABLE: u32 = 0x00fe_0e84;
        const TIMING_POOL_GLOBAL: u32 = 0x0167_E2A0;
        const DETACH_TIMING_NODE: u32 = 1;
        const DESTROY_COMPLEX_BASE: u32 = 2;

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

        let timing_node_a_slot = this.wrapping_add(TIMING_NODE_A);
        let timing_node_a = read_u32(timing_node_a_slot);
        if timing_node_a != 0 {
            let _ = lf_checker_rt::callee_thiscall!(
                DETACH_TIMING_NODE,
                u32,
                timing_node_a,
                timing_node_a_slot
            );
        }

        let timing_node_b_slot = this.wrapping_add(TIMING_NODE_B);
        let timing_node_b = read_u32(timing_node_b_slot);
        if timing_node_b != 0 {
            let _ = lf_checker_rt::callee_thiscall!(
                DETACH_TIMING_NODE,
                u32,
                timing_node_b,
                timing_node_b_slot
            );
        }

        let _ = lf_checker_rt::callee_thiscall!(DESTROY_COMPLEX_BASE, u32, this);

        if flags & 1 != 0 {
            let timing_pool = *lf_checker_rt::global::<u32>(TIMING_POOL_GLOBAL);
            let pool_base = read_u32(timing_pool);
            let map = read_u32(timing_pool + 4);
            let stride = read_u32(timing_pool + 12) as i32;
            let quotient = this.wrapping_sub(pool_base) as i32 / stride;
            let map_byte = map.wrapping_add(quotient as u32);
            write_u8(map_byte, read_u8(map_byte) | TIMING_MAP);

            let smallest_index_slot = timing_pool + 16;
            let smallest_index = read_u32(smallest_index_slot) as i32;
            if quotient < smallest_index {
                write_u32(smallest_index_slot, quotient as u32);
            }
            let live_count_slot = timing_pool + 20;
            write_u32(live_count_slot, read_u32(live_count_slot).wrapping_sub(1));
        }

        this
    }
});
