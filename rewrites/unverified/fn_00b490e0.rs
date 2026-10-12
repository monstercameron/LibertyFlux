// original: 0x00B490E0 CEventRanOverPed::vf8

/// Whether the ran-over-ped event affects a ped: the ped must be on foot or in a matching seat state, have an active task, its virtual status check must decline, it must not be of the excluded kind, and it must be flagged aboard.
///
/// Indirect calls go through fabricated heap objects exactly like the
/// original, landing on the contract's planted stubs.
lf_checker_rt::export!(thiscall, rw_00b490e0(this: u32, ped: u32) -> u32 {

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    unsafe {
        if read_u8(ped.wrapping_add(0x210)) != 0 {
            let seat = read_u32(ped.wrapping_add(0x0A74));
            if seat == 1 || seat == 2 {
                return 0;
            }
        }
        if read_u32(ped.wrapping_add(0x224)) == 0 {
            return 0;
        }
        let vtable = read_u32(ped);
        let slot = read_u32(vtable.wrapping_add(0x128));
        let status: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
        if status(ped) & 0xFF != 0 {
            return 0;
        }
        if read_u8(ped.wrapping_add(0x0A60)) == 2 {
            return 0;
        }
        if read_u8(ped.wrapping_add(0x26C)) & 4 == 0 {
            return 0;
        }
        1
    }
});
