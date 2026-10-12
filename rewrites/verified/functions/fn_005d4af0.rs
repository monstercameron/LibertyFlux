// original: 0x005d4af0 CViewportRadar::vf12
//
// Configure the radar viewport's two overlay slots. The incoming stack word
// is forwarded to the base method. The allocator object is obtained from TLS
// slot 0 and invoked through vtable slot 2 with a size tag, alignment and
// zero flags. Each non-null allocation is adapted before it is registered at
// `this + 0x400`; a null allocation registers null. The final slot call's
// result is returned. Convention: thiscall with one stack word.
lf_checker_rt::export!(thiscall, rw_005d4af0(this: u32, request_flags: u32) -> u32 {
    unsafe {
        const ID_BASE_SETUP: u32 = 1;
        const ID_ADAPT_FIRST: u32 = 3;
        const ID_REGISTER: u32 = 4;
        const ID_ADAPT_SECOND: u32 = 5;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(ID_BASE_SETUP, u32, this, request_flags);

        let tls_base = lf_checker_rt::tls_slot(0);
        let allocator = read_u32(tls_base.wrapping_add(8));
        let allocator_vtable = read_u32(allocator);
        let allocate_address = read_u32(allocator_vtable.wrapping_add(8));
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocate_address as usize);

        let first_allocation = allocate(allocator, 0x950, 0x10, 0);
        let first_value = if first_allocation != 0 {
            lf_checker_rt::callee_thiscall!(ID_ADAPT_FIRST, u32, first_allocation, this)
        } else {
            0
        };
        let register_slot = this.wrapping_add(0x400);
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_REGISTER, u32, register_slot, first_value);

        let allocator_again = read_u32(tls_base.wrapping_add(8));
        let allocator_vtable_again = read_u32(allocator_again);
        let allocate_address_again = read_u32(allocator_vtable_again.wrapping_add(8));
        let allocate_again: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocate_address_again as usize);
        let second_allocation = allocate_again(allocator_again, 0x940, 0x10, 0);
        if second_allocation != 0 {
            let second_value: u32 = lf_checker_rt::callee_thiscall!(
                ID_ADAPT_SECOND, u32, second_allocation, this
            );
            lf_checker_rt::callee_thiscall!(ID_REGISTER, u32, register_slot, second_value)
        } else {
            lf_checker_rt::callee_thiscall!(ID_REGISTER, u32, register_slot, 0)
        }
    }
});
