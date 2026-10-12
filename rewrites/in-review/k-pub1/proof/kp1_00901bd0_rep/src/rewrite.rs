// original: 0x00901bd0 input_device_rebind (proposed)
/// Rebind the shared device through its virtual table when stale.
///
/// Returns entry `eax` without doing anything when the mode word is 1;
/// the generation word when the two generation words disagree or the state
/// word already holds the ready mode. Otherwise the shared device's vtable slot 8 is
/// called and then its slot 4 is tail-called (both thiscalls on the
/// device), and the tail answer is returned. Cdecl with no arguments.
export!(cdecl, rw_00901bd0() -> u32 {
    unsafe {
        /// Mode word (file VA).
        const MODE: u32 = 0x011F7060;
        /// Generation words that must match (file VAs).
        const GEN_A: u32 = 0x012088B4;
        const GEN_B: u32 = 0x00F1C040;
        /// State word holding the ready mode (file VA).
        const STATE: u32 = 0x01037720;
        /// Ready-mode value.
        const READY: u32 = 0x12;
        /// Shared device word (file VA).
        const DEVICE: u32 = 0x0118D808;
        const SLOT_CALL: u32 = 8;
        const SLOT_TAIL: u32 = 4;
        if (global::<u32>(MODE)).read_unaligned() == 1 {
            return 0;
        }
        // Both later early paths return the generation word still in eax.
        let gen = (global::<u32>(GEN_A)).read_unaligned();
        if gen != (global::<u32>(GEN_B)).read_unaligned() {
            return gen;
        }
        if (global::<u32>(STATE)).read_unaligned() == READY {
            return gen;
        }
        let dev = (global::<u32>(DEVICE)).read_unaligned();
        let vt = (dev as *const u32).read_unaligned();
        let a1 = ((vt.wrapping_add(SLOT_CALL)) as *const u32).read_unaligned();
        let f1: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(a1 as usize);
        let _: u32 = f1(dev);
        let a2 = ((vt.wrapping_add(SLOT_TAIL)) as *const u32).read_unaligned();
        let f2: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(a2 as usize);
        f2(dev)
    }
});
