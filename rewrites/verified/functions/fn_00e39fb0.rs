// original: 0x00E39FB0 ui_update_montage_player_visibility

/// Notify the option owned by the player, then notify both child controls
/// through their slot-`0x120` virtual methods. Finally clear the player's
/// visibility-change byte and return the second child call's result. The
/// method is thiscall; each notification is a one-word thiscall.
lf_checker_rt::export!(thiscall, rw_00e39fb0(this: u32) -> u32 {
    const OWNER_OPTION_OFFSET: u32 = 0x22C;
    const SECOND_CHILD_OFFSET: u32 = 0x1E4;
    const FIRST_CHILD_OFFSET: u32 = 0x1EC;
    const VISIBILITY_CHANGED_OFFSET: u32 = 0x23A;
    const VIRTUAL_SLOT_OFFSET: u32 = 0x120;

    unsafe {
        let owner_option = (this.wrapping_add(OWNER_OPTION_OFFSET) as *const u32)
            .read_unaligned();
        let _owner_result = lf_checker_rt::callee_thiscall!(1, u32, owner_option, 1);

        let first_child = (this.wrapping_add(FIRST_CHILD_OFFSET) as *const u32)
            .read_unaligned();
        let first_target = (first_child as *const u32).read_unaligned();
        let first_vtable_entry = (first_target.wrapping_add(VIRTUAL_SLOT_OFFSET) as *const u32)
            .read_unaligned();
        let first_method: unsafe extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(first_vtable_entry as usize);
        let _first_result = first_method(first_child, 0);

        let second_child = (this.wrapping_add(SECOND_CHILD_OFFSET) as *const u32)
            .read_unaligned();
        let second_target = (second_child as *const u32).read_unaligned();
        let second_vtable_entry = (second_target.wrapping_add(VIRTUAL_SLOT_OFFSET) as *const u32)
            .read_unaligned();
        let second_method: unsafe extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(second_vtable_entry as usize);
        let second_result = second_method(second_child, 1);

        (this.wrapping_add(VISIBILITY_CHANGED_OFFSET) as *mut u8).write(0);
        second_result
    }
});
