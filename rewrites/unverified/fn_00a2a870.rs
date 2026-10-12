// original: 0x00A2A870 CPlayerPed::acquire_aim_target
/// Acquires an aim target from the ped's targeting context, optionally
/// scales the returned threshold by the active aim factor, then creates a
/// target object and marks the live inner context as selected.
const CALL_GET_CONTEXT: u32 = 1;
const CALL_GET_PROPERTY: u32 = 2;
const CALL_STATE_GATE: u32 = 3;
const CALL_CREATE_TARGET: u32 = 4;
const CALL_ATTACH_TARGET: u32 = 5;
const STATE_FILE_VA: u32 = 0x011D_6FD4;
const CONTEXT_OFFSET: usize = 0x228;
const CONTEXT_FLOAT_OFFSET: usize = 0x5B4;
const CONTEXT_FLAG_OFFSET: usize = 0x485;

#[inline(always)]
unsafe fn read_u32(base: *const u8, offset: usize) -> u32 {
    // SAFETY: callers pass a heap object supplied by the contract fixture.
    unsafe { base.add(offset).cast::<u32>().read_unaligned() }
}

#[inline(always)]
unsafe fn read_f32(base: *const u8, offset: usize) -> f32 {
    // SAFETY: callers pass a heap object containing the requested float field.
    unsafe { base.add(offset).cast::<f32>().read_unaligned() }
}

lf_checker_rt::export!(thiscall, fn_00a2a870(
    ped: *mut u8,
    property_key: u32,
    helper_flag: u32,
) -> u8 {
    unsafe {
        let ped_address = ped as u32;
        let targeting_component = ped_address.wrapping_add(0x2B0);
        let context_result = lf_checker_rt::callee_thiscall!(
            CALL_GET_CONTEXT,
            u32,
            targeting_component,
        );
        if context_result == 0 {
            return 0;
        }

        let property_argument = read_u32(context_result as *const u8, 0x18);
        let property_result = lf_checker_rt::callee_cdecl!(
            CALL_GET_PROPERTY,
            u32,
            property_argument,
        );
        let initial_threshold = read_f32(property_result as *const u8, 0x14);
        let state = lf_checker_rt::global::<u32>(STATE_FILE_VA).read_unaligned();
        let threshold = if state == 2
            && lf_checker_rt::callee_cdecl!(CALL_STATE_GATE, u8,) != 0
        {
            let context = read_u32(ped, CONTEXT_OFFSET) as *const u8;
            if context.is_null() {
                initial_threshold
            } else {
                let aim_factor = read_f32(context, CONTEXT_FLOAT_OFFSET);
                let left = core::hint::black_box(initial_threshold);
                let right = core::hint::black_box(aim_factor);
                left * right
            }
        } else {
            initial_threshold
        };

        let helper_code = if helper_flag as u8 == 0 { 0x22E8 } else { 0x22E4 };
        let target = lf_checker_rt::callee_thiscall!(
            CALL_CREATE_TARGET,
            u32,
            ped_address.wrapping_add(0xE6C),
            helper_code,
            threshold.to_bits(),
            property_key,
            0u32,
            0u32,
        );
        if target == 0 {
            return 0;
        }

        let _ = lf_checker_rt::callee_thiscall!(
            CALL_ATTACH_TARGET,
            u32,
            targeting_component,
            target,
        );
        let context = read_u32(ped, CONTEXT_OFFSET) as *mut u8;
        if context.is_null() {
            return 0;
        }
        context.add(CONTEXT_FLAG_OFFSET).write(1);
        1
    }
});
