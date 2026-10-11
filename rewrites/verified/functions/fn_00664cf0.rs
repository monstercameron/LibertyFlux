// original: 0x00664CF0 rage::snSendInvitesTask::vf2

/// Initializes a first-run flag, tries the active helper for states two and
/// three, then falls back to the task's virtual callback.
///
/// A zero dword at `this + 0x0c` is changed to one. The active object pointer
/// at `this + 0x60` selects a signed state word at its `+0x50`: only values 2
/// and 3 call the direct helper. That helper receives the object at `+0x48` in
/// ECX, the dwords at `this + 0x98` and `this + 0x9c`, and pointers to fields
/// at `+0xa0`, `+0xb0`, and `+0x90` in that order. A nonzero low byte returns
/// the helper's EAX value. Otherwise the method calls vtable slot `+0x1c` with
/// two zero arguments and returns its EAX value. The method is thiscall with
/// no explicit stack arguments; both callees are thiscall.
lf_checker_rt::export!(thiscall, rw_00664cf0(this: u32) -> u32 {
    unsafe {
        const FIRST_RUN_FLAG: u32 = 0x0c;
        const ACTIVE_OBJECT: u32 = 0x60;
        const STATE_WORD_FROM_HELPER: u32 = 0x08;
        const CALLBACK_SLOT: u32 = 0x1c;
        const HELP_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn invoke_callback(this: u32) -> u32 {
            let vtable = unsafe { read_word(this) };
            let target = unsafe { read_word(vtable.wrapping_add(CALLBACK_SLOT)) };
            let callback: extern "thiscall" fn(u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            callback(this, 0, 0)
        }

        if unsafe { read_word(this.wrapping_add(FIRST_RUN_FLAG)) } == 0 {
            unsafe { write_word(this.wrapping_add(FIRST_RUN_FLAG), 1) };
        }

        let active_object = unsafe { read_word(this.wrapping_add(ACTIVE_OBJECT)) };
        let helper_this = active_object.wrapping_add(0x48);
        let helper_state = unsafe {
            read_word(helper_this.wrapping_add(STATE_WORD_FROM_HELPER)) as i32
        };
        if (2..=3).contains(&helper_state) {
            let first_value = unsafe { read_word(this.wrapping_add(0x98)) };
            let second_value = unsafe { read_word(this.wrapping_add(0x9c)) };
            let helper_result: u32 = lf_checker_rt::callee_thiscall!(
                HELP_CALLEE,
                u32,
                helper_this,
                first_value,
                second_value,
                this.wrapping_add(0xa0),
                this.wrapping_add(0xb0),
                this.wrapping_add(0x90)
            );
            if helper_result as u8 != 0 {
                return helper_result;
            }
        }
        unsafe { invoke_callback(this) }
    }
});
