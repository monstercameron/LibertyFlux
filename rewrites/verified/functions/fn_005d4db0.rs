// original: 0x005d4db0 CViewportFrontend3DScene::vf12
//
// Create and register the frontend render phase. The incoming stack word is
// forwarded to the base method. The allocator stored in TLS slot 0 is called
// through vtable slot 2 for an object of size 0x950. If allocation succeeds,
// the phase constructor runs, then the new object receives its vtable, phase
// tag and three colour words before it is registered at `this + 0x400`. A
// null allocation registers null. The registration result is returned.
// Convention: thiscall with one stack word.
lf_checker_rt::export!(thiscall, rw_005d4db0(this: u32, request_flags: u32) -> u32 {
    unsafe {
        const ID_BASE_SETUP: u32 = 1;
        const ID_PHASE_CONSTRUCT: u32 = 3;
        const ID_REGISTER: u32 = 4;
        const PHASE_VTABLE_VA: u32 = 0x00fe_1094;

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
        let phase = allocate(allocator, 0x950, 0x10, 0);
        let register_slot = this.wrapping_add(0x400);

        if phase == 0 {
            lf_checker_rt::callee_thiscall!(ID_REGISTER, u32, register_slot, 0)
        } else {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                ID_PHASE_CONSTRUCT, u32, phase, this
            );
            write_u32(phase, lf_checker_rt::relocated(PHASE_VTABLE_VA));
            write_u32(phase.wrapping_add(0x8f4), 6);
            (phase.wrapping_add(0x1c) as *mut u8).write(1);
            write_u32(phase.wrapping_add(0x940), 0xbf80_0000);
            write_u32(phase.wrapping_add(0x944), 0xbf80_0000);
            write_u32(phase.wrapping_add(0x948), 0);
            lf_checker_rt::callee_thiscall!(ID_REGISTER, u32, register_slot, phase)
        }
    }
});
