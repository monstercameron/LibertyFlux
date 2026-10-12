// original: 0x005EFD40 CTaskComplexCarDriveMissionKillPed::vf0

/// Destroy the object and, when flagged, release its pool slot.
///
/// Installs the vtable and runs the base destructor, then returns `this`
/// unless bit 0 of the stack flag is set. When set, releases the slot back
/// to the shared pool: the slot index is (`this` minus the pool base,
/// wrapping) divided by the slot size as SIGNED 32-bit values, the slot's
/// used bit (bit 7 of its byte in the pool bit array) is set, the pool's low
/// free hint is lowered past the index when the index is smaller (a signed
/// comparison), and the pool's live count is decremented. The original is
/// thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_005efd40(this: u32, flag: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EB35E4;
        const POOL_GLOBAL: u32 = 0x0167E2A0;
        const BASE_CALLEE: u32 = 1;
        const BITS_OFFSET: u32 = 0x04;
        const SIZE_OFFSET: u32 = 0x0c;
        const LOW_OFFSET: u32 = 0x10;
        const COUNT_OFFSET: u32 = 0x14;
        const USED_BIT: u8 = 0x80;
        (this as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, this);
        if flag & 1 == 0 {
            return this;
        }
        let pool =
            (lf_checker_rt::global::<u32>(POOL_GLOBAL)).read_unaligned();
        let base = (pool as *const u32).read_unaligned();
        let bits = (pool.wrapping_add(BITS_OFFSET) as *const u32)
            .read_unaligned();
        let size = (pool.wrapping_add(SIZE_OFFSET) as *const u32)
            .read_unaligned();
        let index =
            (this.wrapping_sub(base) as i32).wrapping_div(size as i32);
        let slot = bits.wrapping_add(index as u32) as *mut u8;
        slot.write_unaligned(slot.read_unaligned() | USED_BIT);
        let low = pool.wrapping_add(LOW_OFFSET) as *mut u32;
        if index < low.read_unaligned() as i32 {
            low.write_unaligned(index as u32);
        }
        let count = pool.wrapping_add(COUNT_OFFSET) as *mut u32;
        count.write_unaligned(count.read_unaligned().wrapping_sub(1));
        this
    }
});
