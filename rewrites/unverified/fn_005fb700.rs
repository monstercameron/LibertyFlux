// original: 0x005FB700 rage::fragInst::vf52

/// Release this fragment instance's resource and update the associated counters.
/// A nonnull resource triggers three manager calls and decrements the manager count.
/// A type index other than `0xffff` is sent to the global tracker. If a referenced
/// type exists and tracking is enabled, its 16-bit count wraps down; when it reaches
/// zero, the notification helper receives that type and flag 1. The instance's type
/// reference pointer is cleared unconditionally. This is a thiscall method.

lf_checker_rt::export!(thiscall, rw_005fb700(this: u32) -> () {
    const RESOURCE_POINTER: u32 = 0x64;
    const TYPE_INDEX: u32 = 0x08;
    const TYPE_REFERENCE: u32 = 0x04;
    const MANAGER_GLOBAL: u32 = 0x018B_7448;
    const TYPE_TRACKER_GLOBAL: u32 = 0x018B_99AC;
    const TRACKER_ENABLED_GLOBAL: u32 = 0x018B_99B8;
    const MANAGER_SERVICE: u32 = 0x2f0;
    const MANAGER_COUNT: u32 = 0x04;
    const TYPE_REFERENCE_COUNT: u32 = 0x1a;

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u16(address: u32, value: u16) {
        unsafe { (address as *mut u16).write_unaligned(value) }
    }

    unsafe {
        let resource = read_u32(this.wrapping_add(RESOURCE_POINTER));
        if resource != 0 {
            let manager = core::ptr::read_unaligned(lf_checker_rt::global::<u32>(MANAGER_GLOBAL));
            let state = read_u32(manager.wrapping_add(MANAGER_SERVICE));
            let _ = lf_checker_rt::callee_thiscall!(1, u32, resource);
            let manager_view = state.wrapping_add(0x0c);
            let _ = lf_checker_rt::callee_thiscall!(2, u32, manager_view, resource);
            let _ = lf_checker_rt::callee_thiscall!(3, u32, manager_view, resource);
            let count = read_u32(state.wrapping_add(MANAGER_COUNT));
            write_u32(state.wrapping_add(MANAGER_COUNT), count.wrapping_sub(1));
        }

        let type_index = u32::from(read_u16(this.wrapping_add(TYPE_INDEX)));
        if type_index != u32::from(u16::MAX) {
            let tracker = core::ptr::read_unaligned(
                lf_checker_rt::global::<u32>(TYPE_TRACKER_GLOBAL),
            );
            let _ = lf_checker_rt::callee_thiscall!(4, u32, tracker, type_index, 0);
        }

        let type_reference = read_u32(this.wrapping_add(TYPE_REFERENCE));
        if type_reference != 0
            && core::ptr::read_unaligned(lf_checker_rt::global::<u8>(TRACKER_ENABLED_GLOBAL)) != 0
        {
            let count_address = type_reference.wrapping_add(TYPE_REFERENCE_COUNT);
            let count = read_u16(count_address).wrapping_sub(1);
            write_u16(count_address, count);
            if count == 0 {
                let _ = lf_checker_rt::callee_thiscall!(5, u32, type_reference, 1);
            }
        }
        write_u32(this.wrapping_add(TYPE_REFERENCE), 0);
    }
}
