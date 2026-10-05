// original: 0x00985650 audEmitterAudioEntity::audEmitterAudioEntity_2
/// Array constructor: release the slot, stamp every element, hand off.
///
/// Takes the object pointer in ECX. Installs the working vtable, releases
/// the slotted pointer through the helper and clears the slot, stamps the
/// element vtable down the whole fixed array, installs the final vtable,
/// and transfers control to the chained base constructor with the same
/// object pointer, returning whatever that call answers.
export!(thiscall, rw_00985650(this: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0x00E8E2CC);
        let slot = *(this.wrapping_add(0x38240) as *const u32);
        callee_cdecl!(2, u32, slot);
        *(this.wrapping_add(0x38240) as *mut u32) = 0;
        let elem = relocated(0x00E8E23C);
        let mut p = this.wrapping_add(0x8240);
        for _ in 0..0xA0u32 {
            p = p.wrapping_sub(0xD0);
            *(p as *mut u32) = elem;
        }
        *(this as *mut u32) = relocated(0x00E83134);
        callee_thiscall!(1, u32, this)
    }
});
