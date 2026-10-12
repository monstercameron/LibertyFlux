// original: 0x009F4160 NativeImpl_REGISTER_HATED_TARGETS_IN_AREA
/// Requires two cdecl-independent thiscall gates before resolving the ped's
/// info object. It returns whether either of two status bits is set in that
/// object. Calls are kept in their original order and use scripted stand-ins.
lf_checker_rt::export!(thiscall, rw_009F4160(ped: u32) -> u8 {
    unsafe {
        const PED_STATE_OBJECT: u32 = 0x224;
        const PED_STATUS_VIEW: u32 = 0x2B0;
        const INFO_STATUS_FLAGS: u32 = 0x8F4;
        const STATUS_MASK: u32 = 0x0006_0000;
        let is_available = lf_checker_rt::callee_thiscall!(1, u32, ped);
        if is_available as u8 == 0 { return 0; }
        let can_register = lf_checker_rt::callee_thiscall!(2, u32, ped.wrapping_add(PED_STATUS_VIEW), 1u32);
        if can_register as u8 == 0 { return 0; }
        let state_object = (ped.wrapping_add(PED_STATE_OBJECT) as *const u32).read_unaligned();
        let info = lf_checker_rt::callee_thiscall!(3, u32, state_object);
        let status = (info.wrapping_add(INFO_STATUS_FLAGS) as *const u32).read_unaligned();
        u8::from(status & STATUS_MASK != 0)
    }
});
