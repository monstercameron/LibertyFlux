// original: 0x008DAAD0 CCreateRenderListGroupDC::CCreateRenderListGroupDC

/// Create a render-list-group command by copying four words from global configuration, advancing the shared serial token, then issuing two intercepted setup calls. The second call receives the scripted first-call result and a table address derived from the global index; the command pointer is returned.
lf_checker_rt::export!(thiscall, rw_008daad0(this: u32) -> u32 {
    const SERIAL_COUNTER_VA: u32 = 0x010327A0;
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81D9C;
    const GROUP_TABLE_THIS_VA: u32 = 0x01175C58;
    const GROUP_TABLE_BASE_VA: u32 = 0x01175CA0;
    const GROUP_MANAGER_VA: u32 = 0x01593318;
    const INDEX_VA: u32 = 0x01175C6C;
    const GLOBAL_FIELD_0_VA: u32 = 0x0159B750;
    const GLOBAL_FIELD_1_VA: u32 = 0x0159B754;
    const GLOBAL_FIELD_2_VA: u32 = 0x0159B758;
    const GLOBAL_FIELD_3_VA: u32 = 0x0159B760;
    const SERIAL_MASK: u32 = 0x0000_3FFF;
    const INITIALIZE_CALLEE: u32 = 85;
    const REGISTER_CALLEE: u32 = 86;
    const INITIALIZE_ARGUMENT: u32 = 0x000C_0000;
    unsafe {
        let command = this as *mut u8;
        let counter = lf_checker_rt::global::<u32>(SERIAL_COUNTER_VA);
        let prior_token = core::ptr::read_unaligned(command.wrapping_add(4).cast::<u32>());
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(BASE_VTABLE_VA));
        let serial = core::ptr::read_volatile(counter.cast_const());
        let token = prior_token ^ ((prior_token ^ serial) & SERIAL_MASK);
        core::ptr::write_unaligned(command.wrapping_add(4).cast::<u32>(), token);
        let serial_after_token = core::ptr::read_volatile(counter.cast_const());
        core::ptr::write_volatile(counter, serial_after_token.wrapping_add(1));
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));

        let value_0 = core::ptr::read_volatile(lf_checker_rt::global::<u32>(GLOBAL_FIELD_0_VA).cast_const());
        core::ptr::write_unaligned(command.wrapping_add(8).cast::<u32>(), value_0);
        let value_1 = core::ptr::read_volatile(lf_checker_rt::global::<u32>(GLOBAL_FIELD_1_VA).cast_const());
        core::ptr::write_unaligned(command.wrapping_add(0x0c).cast::<u32>(), value_1);
        let value_2 = core::ptr::read_volatile(lf_checker_rt::global::<u32>(GLOBAL_FIELD_2_VA).cast_const());
        core::ptr::write_unaligned(command.wrapping_add(0x10).cast::<u32>(), value_2);
        let value_3 = core::ptr::read_volatile(lf_checker_rt::global::<u32>(GLOBAL_FIELD_3_VA).cast_const());
        core::ptr::write_unaligned(command.wrapping_add(0x14).cast::<u32>(), value_3);

        let first_result = lf_checker_rt::callee_thiscall!(INITIALIZE_CALLEE, u32, lf_checker_rt::relocated(GROUP_TABLE_THIS_VA), 0, INITIALIZE_ARGUMENT);
        let group_index = core::ptr::read_volatile(lf_checker_rt::global::<u32>(INDEX_VA).cast_const());
        let table_slot = group_index.wrapping_shl(10).wrapping_add(GROUP_TABLE_BASE_VA);
        let _ = lf_checker_rt::callee_thiscall!(REGISTER_CALLEE, u32, lf_checker_rt::relocated(GROUP_MANAGER_VA), first_result, INITIALIZE_ARGUMENT, table_slot, 0x80);
    }
    this
});
