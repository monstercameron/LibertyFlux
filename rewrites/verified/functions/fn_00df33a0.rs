// original: 0x00DF33A0 ui_dispatch::clear_and_enable_controls_if_unhandled

/// Ask the object's virtual action gate at slot `0x140`. A nonzero result is
/// returned immediately. On zero, clear the state word at `this + 0x1bc`, run
/// the no-argument follow-up at slot `0x1ec`, then invoke the three one-argument
/// control hooks at slots `0x28`, `0x24`, and `0x13c` in that order. The final
/// hook's full `EAX` result is returned. All targets remain virtual so the
/// object's function table determines the behavior.
lf_checker_rt::export!(thiscall, rw_00df33a0(this: u32) -> u32 {
    unsafe {
        const OBJECT_TABLE: u32 = 0;
        const ACTION_GATE_SLOT: u32 = 0x140;
        const FOLLOWUP_SLOT: u32 = 0x1ec;
        const PRIMARY_HOOK_SLOT: u32 = 0x28;
        const SECONDARY_HOOK_SLOT: u32 = 0x24;
        const FINAL_HOOK_SLOT: u32 = 0x13c;
        const STATE_WORD: u32 = 0x1bc;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn call_no_args(table: u32, slot: u32, object: u32) -> u32 {
            let address = read_u32(table.wrapping_add(slot));
            let method: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(address as usize);
            method(object)
        }

        #[inline(always)]
        unsafe fn call_with_one(table: u32, slot: u32, object: u32, value: u32) -> u32 {
            let address = read_u32(table.wrapping_add(slot));
            let method: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(address as usize);
            method(object, value)
        }

        let object_table = read_u32(this.wrapping_add(OBJECT_TABLE));
        let gate_result = call_no_args(object_table, ACTION_GATE_SLOT, this);
        if gate_result as u8 != 0 {
            return gate_result;
        }

        (this.wrapping_add(STATE_WORD) as *mut u32).write_unaligned(0);
        let _ = call_no_args(object_table, FOLLOWUP_SLOT, this);
        let _ = call_with_one(object_table, PRIMARY_HOOK_SLOT, this, 1);
        let _ = call_with_one(object_table, SECONDARY_HOOK_SLOT, this, 1);
        call_with_one(object_table, FINAL_HOOK_SLOT, this, 1)
    }
});
