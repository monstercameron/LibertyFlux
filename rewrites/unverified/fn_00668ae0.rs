// original: 0x00668AE0 ptx_event_emitter_release_components

/// For a non-null event at +0x48, reads its virtual slot +0x1c and passes the result to the component at +0x40 through the first direct helper. It calls the paired second helper when the first returns AL=0. It repeats the process for the event at +0x4c using slot +0x68 and the component at +0x44. Null events contribute a zero helper argument. The function has no defined return value; the contract compares every outgoing call, stack adjustment, and fault.
lf_checker_rt::export!(thiscall, rw_00668AE0(this: u32) -> u32 {
    const FIRST_EVENT: u32 = 0x48;
    const SECOND_EVENT: u32 = 0x4c;
    const FIRST_COMPONENT: u32 = 0x40;
    const SECOND_COMPONENT: u32 = 0x44;
    const FIRST_SLOT: u32 = 0x1c;
    const SECOND_SLOT: u32 = 0x68;
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn vcall0(object: u32, slot: u32) -> u32 {
        let vtable = unsafe { read_u32(object) };
        let target = unsafe { read_u32(vtable.wrapping_add(slot)) };
        let method: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        method(object)
    }
    let first_event = unsafe { read_u32(this.wrapping_add(FIRST_EVENT)) };
    let first_value = if first_event == 0 { 0 } else { unsafe { vcall0(first_event, FIRST_SLOT) } };
    let first_component = this.wrapping_add(FIRST_COMPONENT);
    let first_accepted = lf_checker_rt::callee_thiscall!(1, u8, first_component, first_value);
    if first_accepted == 0 {
        let _ignored_result = lf_checker_rt::callee_thiscall!(2, u32, first_component, first_value);
    }
    let second_event = unsafe { read_u32(this.wrapping_add(SECOND_EVENT)) };
    let second_value = if second_event == 0 { 0 } else { unsafe { vcall0(second_event, SECOND_SLOT) } };
    let second_component = this.wrapping_add(SECOND_COMPONENT);
    let second_accepted = lf_checker_rt::callee_thiscall!(1, u8, second_component, second_value);
    if second_accepted == 0 {
        let _ignored_result = lf_checker_rt::callee_thiscall!(2, u32, second_component, second_value);
    }
    0
});
