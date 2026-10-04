// original: 0x00b245a0 CPhysical::vf68
// s08_b245a0 (CPhysical::vf68): detach from the attached entity.
// thiscall/0: if the attached-entity slot at +0x1BC is occupied and that
// entity's type tag at +0x28 has bits 6-7 set with bits 8-9 clear, notify
// it (thiscall/1); then release the slot through the shared release helper
// (thiscall/1), mark both link words empty (-1 / 0) and clear the slot.
// Returns 0.
export!(thiscall, rw_b245a0(this: *mut u8) -> u32 {
    unsafe {
        let slot = this.add(0x1BC) as *mut u32;
        let attached = *slot;
        if attached != 0 && (*((attached + 0x28) as *const u32) & 0x3C0) == 0xC0 {
            let notify: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(callee_addr(1) as usize);
            notify(attached, this as u32);
        }
        if *slot != 0 {
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(callee_addr(2) as usize);
            release(*slot, slot as u32);
        }
        *(this.add(0x1E0) as *mut u16) = 0xFFFF;
        *(this.add(0x1E2) as *mut u16) = 0;
        *slot = 0;
        0
    }
});
