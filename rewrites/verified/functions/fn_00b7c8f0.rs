// original: 0x00b7c8f0 CTaskComplexDrivePointRoute::vf19

/// Create a route-drive task from the shared task pool. If allocation
/// fails, return zero. Otherwise forward the source task's route fields to
/// the new object's constructor. The route index is incremented with 32-bit
/// wrapping, shifted left four bits, and added to the route base. The byte at
/// `+0x3c` is sign-extended; the byte at `+0x3b` is zero-extended. The value
/// at `+0x50` is forwarded as its exact 32-bit float representation.
///
/// Original ABI: thiscall with one unused stack argument. The constructor
/// call has nine stack arguments; all are compared by the contract.
lf_checker_rt::export!(thiscall, rw_00b7c8f0(this: u32, _unused: u32) -> u32 {
    unsafe {
        const TASK_POOL_SLOT: u32 = 0x0167e2a0;
        const ROUTE_KIND: u32 = 0x40;
        const ROUTE_BASE: u32 = 0x44;
        const ROUTE_X: u32 = 0x48;
        const ROUTE_Y: u32 = 0x4c;
        const WEIGHT_BITS: u32 = 0x50;
        const ROUTE_INDEX: u32 = 0x54;
        const ROUTE_FLAGS: u32 = 0x3b;
        const ROUTE_CLASS: u32 = 0x3c;

        let pool = lf_checker_rt::global::<u32>(TASK_POOL_SLOT).read();
        let allocated: u32 = lf_checker_rt::callee_thiscall!(1, u32, pool);
        if allocated == 0 {
            return 0;
        }

        let route_kind = ((this + ROUTE_KIND) as *const u32).read_unaligned();
        let route_base = ((this + ROUTE_BASE) as *const u32).read_unaligned();
        let route_x = ((this + ROUTE_X) as *const u32).read_unaligned();
        let route_y = ((this + ROUTE_Y) as *const u32).read_unaligned();
        let route_weight_bits = ((this + WEIGHT_BITS) as *const u32).read_unaligned();
        let route_index = ((this + ROUTE_INDEX) as *const u32).read_unaligned();
        let route_index_offset = route_index.wrapping_add(1).wrapping_shl(4);
        let route_pointer = route_base.wrapping_add(route_index_offset);
        let route_flags = ((this + ROUTE_FLAGS) as *const u8).read() as u32;
        let route_class = ((this + ROUTE_CLASS) as *const i8).read() as i32 as u32;

        lf_checker_rt::callee_thiscall!(2, u32, allocated, route_kind,
            route_pointer, route_flags, route_x, route_y, route_weight_bits,
            route_class, 0x14, 1)
    }
});
