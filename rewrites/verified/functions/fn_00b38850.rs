// original: 0x00B38850 object_reconfigure
/// Queries an object's three virtual configuration views, conditionally
/// refreshes its configuration pointer, forwards two float settings to the
/// update helper, and optionally releases the object through a final helper.
const CALL_VIEW: u32 = 1;
const CALL_VALIDATE: u32 = 2;
const CALL_MODE: u32 = 3;
const CALL_LEVEL: u32 = 4;
const CALL_LOOKUP: u32 = 5;
const CALL_UPDATE: u32 = 6;
const CALL_RELEASE: u32 = 7;
const STATE_FILE_VA: u32 = 0x011D_6FD4;
const FLOAT_A_FILE_VA: u32 = 0x00FE_881C;
const FLOAT_B_FILE_VA: u32 = 0x00FE_87E4;
const FLOAT_SPECIAL_FILE_VA: u32 = 0x00FE_88E8;
const LOOKUP_TABLE_FILE_VA: u32 = 0x0171_F994;
const OBJECT_CONFIGURATION_OFFSET: usize = 0x138;
const VIEW_CATEGORY_OFFSET: usize = 0x12C;

#[inline(always)]
unsafe fn read_u32(base: *const u8, offset: usize) -> u32 {
    // SAFETY: the caller passes an object from the contract's live heap fixture.
    unsafe { base.add(offset).cast::<u32>().read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(base: *mut u8, offset: usize, value: u32) {
    // SAFETY: the caller passes the writable object supplied as the first argument.
    unsafe { base.add(offset).cast::<u32>().write_unaligned(value) };
}

#[inline(always)]
unsafe fn read_image_float(file_va: u32) -> f32 {
    // SAFETY: each file address is a read-only float operand used by this routine.
    unsafe { lf_checker_rt::global::<f32>(file_va).read_unaligned() }
}

#[inline(always)]
unsafe fn refresh_configuration(object: *mut u8, category: u32) {
    let table_base = lf_checker_rt::relocated(LOOKUP_TABLE_FILE_VA);
    let slot = table_base.wrapping_add(category.wrapping_mul(20));
    let value = lf_checker_rt::callee_thiscall!(CALL_LOOKUP, u32, slot, 0u32);
    if value != 0 {
        unsafe { write_u32(object, OBJECT_CONFIGURATION_OFFSET, value) };
    }
}

lf_checker_rt::export!(cdecl, fn_00b38850(
    object: *mut u8,
    validate_flag: u32,
    update_flag: u32,
    release_flag: u32,
) -> () {
    unsafe {
        let object_address = object as u32;

        let first_view = lf_checker_rt::callee_thiscall!(CALL_VIEW, u32, object_address);
        let category = read_u32(first_view as *const u8, VIEW_CATEGORY_OFFSET);
        let second_view = lf_checker_rt::callee_thiscall!(CALL_VIEW, u32, object_address);
        let lookup_context = second_view.wrapping_add(0x80);
        let third_view = lf_checker_rt::callee_thiscall!(CALL_VIEW, u32, object_address);

        if validate_flag as u8 != 0
            && lf_checker_rt::callee_cdecl!(CALL_VALIDATE, u8, category) != 0
        {
            let state = lf_checker_rt::global::<u32>(STATE_FILE_VA).read_unaligned();
            let use_mode_level = state == 3
                && lf_checker_rt::callee_cdecl!(CALL_MODE, u8,) != 0;
            if use_mode_level {
                let level = lf_checker_rt::callee_cdecl!(CALL_LEVEL, u32, category);
                if level != 0 {
                    write_u32(object, OBJECT_CONFIGURATION_OFFSET, level);
                } else {
                    refresh_configuration(object, category);
                }
            } else {
                refresh_configuration(object, category);
            }
        }

        if update_flag as u8 != 0 {
            let base_float = read_image_float(FLOAT_A_FILE_VA);
            let second_float = read_image_float(FLOAT_B_FILE_VA);
            let (first_setting, second_setting, special_category) = if category == 2 {
                (
                    read_image_float(FLOAT_SPECIAL_FILE_VA),
                    base_float,
                    category,
                )
            } else {
                (base_float, second_float, 0)
            };

            let _ = lf_checker_rt::callee_cdecl!(
                CALL_UPDATE,
                u32,
                object_address,
                lookup_context,
                third_view,
                first_setting.to_bits(),
                second_setting.to_bits(),
                special_category,
                0u32,
            );
        }

        if release_flag as u8 != 0 {
            let _ = lf_checker_rt::callee_cdecl!(CALL_RELEASE, u32, object_address);
        }
    }
});
