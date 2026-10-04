// original: 0x00934c10 teardown_slot
/// 0x00934C10: tear down slot `idx`: resolve it, unregister both of its
/// addresses from the engine registry, and when the slot is live, also
/// unregister its payload and body. Returns the registry's last answer.
// shared layout constant
const REGISTRY: u32 = 0x110c0a0;
// shared layout constant
const SCAN_STRIDE: u32 = 0x22c;
// shared layout constant
const SET_RECORDS: usize = 0x144;
export!(thiscall, rw_00934c10(this: *const u8, idx: u32) -> u32 {
    callee_thiscall!(1, u32, this as u32);
    callee_thiscall!(2, u32, this as u32);
    callee_thiscall!(3, u32, this as u32, idx);
    let resolved = callee_thiscall!(4, u32, this as u32, idx);
    callee_thiscall!(
        6,
        u32,
        relocated(REGISTRY),
        resolved.wrapping_add(0x144)
    );
    callee_cdecl!(7, u32, 0u32);
    let slot = callee_thiscall!(5, u32, this as u32, idx);
    if unsafe { core::ptr::read(slot.wrapping_add(0x144) as *const u8) } != 0 {
        let base =
            unsafe { core::ptr::read_unaligned(this.add(SET_RECORDS) as *const u32) };
        let rec = base.wrapping_add(idx.wrapping_mul(SCAN_STRIDE));
        callee_thiscall!(6, u32, relocated(REGISTRY), rec.wrapping_add(0x64));
        callee_cdecl!(8, u32, rec.wrapping_add(0xa4), 0u32);
        callee_thiscall!(9, u32, relocated(REGISTRY));
    }
    callee_thiscall!(9, u32, relocated(REGISTRY))
});
