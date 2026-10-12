// original: 0x005d4c30 CViewportMobilePhone::vf12
//
// Build and register the mobile-phone render phase. The incoming request
// word is forwarded to the base method. A render object is requested from
// the allocator in TLS slot 0 through its vtable slot 2. A live result is
// constructed, given its vtable and flags, and has two viewport children
// initialized before it is registered at `this + 0x400`; a null result
// registers null. The final registration result is returned. Convention:
// thiscall with one stack word. The function preserves its incoming ESI
// value around the phase constructor; the constructor receives only `this`
// as its stack argument.
lf_checker_rt::export!(thiscall, rw_005d4c30(this: u32, request_flags: u32) -> u32 {
    unsafe {
        const ID_BASE_SETUP: u32 = 1;
        const ID_PHASE_INITIALIZE: u32 = 3;
        const ID_REGISTER: u32 = 4;
        const ID_PHASE_CONSTRUCT: u32 = 5;
        const PHASE_VTABLE_VA: u32 = 0x00fe_1014;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(ID_BASE_SETUP, u32, this, request_flags);
        let tls_base = lf_checker_rt::tls_slot(0);
        let allocator = read_u32(tls_base.wrapping_add(8));
        let allocator_vtable = read_u32(allocator);
        let allocate_address = read_u32(allocator_vtable.wrapping_add(8));
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocate_address as usize);
        let phase = allocate(allocator, 0x940, 0x10, 0);

        let render_phase = this.wrapping_add(0x10);
        let register_slot = this.wrapping_add(0x400);
        if phase == 0 {
            lf_checker_rt::callee_thiscall!(ID_REGISTER, u32, register_slot, 0)
        } else {
            let _ = lf_checker_rt::callee_thiscall!(ID_PHASE_CONSTRUCT, u32, phase, this);
            write_u32(phase, lf_checker_rt::relocated(PHASE_VTABLE_VA));
            write_u32(phase.wrapping_add(0x8f4), 6);
            (phase.wrapping_add(0x1c) as *mut u8).write(1);

            let child_a = phase.wrapping_add(0xb0);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                ID_PHASE_INITIALIZE, u32, child_a, render_phase
            );
            (phase.wrapping_add(0x1c) as *mut u8).write(1);

            let child_b = phase.wrapping_add(0x4a0);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                ID_PHASE_INITIALIZE, u32, child_b, render_phase
            );
            lf_checker_rt::callee_thiscall!(ID_REGISTER, u32, register_slot, phase)
        }
    }
});
