// original: 0x00B48900 CEventOnFire::vf8

/// Whether the on-fire event affects a ped: the ped must carry the burning flag, not be flagged dead or extinguished, no active task may report the burning state nor the extinguished state, and the final helper must decline.
///
/// `this` points to the event, `ped` to the candidate ped. Direct helpers
/// are intercepted by the contract; indirect calls go through fabricated
/// heap objects exactly like the original. Returns 1 when the event
/// affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b48900(this: u32, ped: u32) -> u32 {
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    unsafe {
        if read_u32(ped.wrapping_add(0x28)) >> 0x15 & 1 == 0 {
            return 0;
        }
        let flags = read_u32(ped.wrapping_add(0x118));
        if flags & 0x80 != 0 {
            return 0;
        }
        if flags & 0x400 != 0 {
            return 0;
        }
        let slots = read_u32(ped.wrapping_add(0x224));
        let mut index = 0u32;
        while index < 5 {
            if read_u32(slots.wrapping_add(index.wrapping_mul(4)).wrapping_add(0x44)) != 0 {
                break;
            }
            index += 1;
        }
        if index < 5 {
            let task = read_u32(slots.wrapping_add(index.wrapping_mul(4)).wrapping_add(0x44));
            let vtable = read_u32(task);
            let slot = read_u32(vtable.wrapping_add(0x0C));
            let kind_of: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
            if kind_of(task) == 0xFA {
                return 0;
            }
        }
        let current = lf_checker_rt::callee_thiscall!(2, u32, slots.wrapping_add(0x44), 4);
        if current != 0 {
            let vtable = read_u32(current);
            let slot = read_u32(vtable.wrapping_add(0x0C));
            let kind_of: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
            if kind_of(current) == 0xFF {
                return 0;
            }
        }
        let out = lf_checker_rt::callee_thiscall!(3, u32, ped);
        (out & 0xFF == 0) as u32
    }
});
