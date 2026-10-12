// original: 0x005C3E60 CPedFactoryNY::vf9

/// Dispatches subtype 0x10 and 0x11 objects through separate initializer
/// helpers, then reports the selected value through a callback at offset
/// +0x44 of the object's output context. The process-wide manager is read via
/// the relocated global accessor. Other subtype values return without calls.
lf_checker_rt::export!(stdcall, rw_005c3e60(target: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL: u32 = 0x0167e2a0;
        const SUBOBJECT_POINTER: u32 = 0x21c;
        const SUBOBJECT_KIND: u32 = 0x12c;
        const OUTPUT_CONTEXT: u32 = 0x224;
        const CALLBACK_OFFSET: u32 = 0x44;
        const ALLOCATE_CALLEE: u32 = 1;
        const INIT_KIND_10_CALLEE: u32 = 2;
        const INIT_KIND_11_CALLEE: u32 = 3;
        const REPORT_CALLEE: u32 = 4;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let subobject = read_word(target + SUBOBJECT_POINTER);
        let kind = read_word(subobject + SUBOBJECT_KIND);
        let initialized_value = if kind == 0x10 || kind == 0x11 {
            let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned();
            let allocated: u32 = lf_checker_rt::callee_thiscall!(ALLOCATE_CALLEE, u32, manager);
            if allocated == 0 {
                0
            } else if kind == 0x10 {
                lf_checker_rt::callee_thiscall!(INIT_KIND_10_CALLEE, u32, allocated, arg1, arg2, arg3)
            } else {
                lf_checker_rt::callee_thiscall!(INIT_KIND_11_CALLEE, u32, allocated, arg1, arg2, arg3)
            }
        } else {
            return 0;
        };

        let context = read_word(target + OUTPUT_CONTEXT);
        lf_checker_rt::callee_thiscall!(
            REPORT_CALLEE,
            u32,
            context.wrapping_add(CALLBACK_OFFSET),
            initialized_value,
            4,
            0,
        )
    }
});
