// original: 0x00E49440 UITitleMenu::initialize_control_selection

/// Ask the object's virtual selection gate and return its result unchanged
/// when the low byte is zero. Otherwise initialize the control at `this +
/// 0x204`, then return the result of virtual slot `0x13c` with a zero argument.
/// The cdecl initializer observes the address of the control without changing
/// this wrapper's stack, and the two virtual calls remain table driven.
lf_checker_rt::export!(thiscall, rw_00e49440(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0;
        const GATE_SLOT: u32 = 0x140;
        const FINAL_SLOT: u32 = 0x13c;
        const CONTROL_OBJECT: u32 = 0x204;
        const INITIALIZER: u32 = 2;

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
        unsafe fn call_one(table: u32, slot: u32, object: u32, arg: u32) -> u32 {
            let address = read_u32(table.wrapping_add(slot));
            let method: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(address as usize);
            method(object, arg)
        }

        let table = read_u32(this.wrapping_add(VTABLE));
        let gate_result = call_no_args(table, GATE_SLOT, this);
        if gate_result as u8 == 0 {
            return gate_result;
        }

        let _ = lf_checker_rt::callee_cdecl!(
            INITIALIZER,
            u32,
            this.wrapping_add(CONTROL_OBJECT),
        );
        call_one(table, FINAL_SLOT, this, 0)
    }
});
