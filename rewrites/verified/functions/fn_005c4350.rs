// original: 0x005C4350 ped_factory_dispatch_service_call

/// Selects an object through a virtual slot and forwards its result to a
/// second thiscall helper. The `this` object arrives in ECX; the first stack
/// argument supplies the virtual-call object and the second supplies its
/// first stack argument. A global flag byte is forwarded as one meaningful
/// low byte; the original's upper argument bytes are stale stack contents.
lf_checker_rt::export!(thiscall, rw_005c4350(
    this: u32,
    object: u32,
    object_value: u32,
    _unused_arg2: u32,
    _unused_arg3: u32
) -> u32 {
    unsafe {
        const FEATURE_FLAGS_GLOBAL: u32 = 0x018ecfa8;
        const SERVICE_VALUE_GLOBAL: u32 = 0x018e5240;
        const VTABLE_POINTER: u32 = 0x00;
        const SERVICE_SLOT: u32 = 0xbc;
        const SERVICE_CALLEE: u32 = 1;
        const FINALIZE_CALLEE: u32 = 2;
        const FIXED_ARGUMENT_A: u32 = 0;
        const FIXED_ARGUMENT_B: u32 = 10;
        const FIXED_ARGUMENT_C: u32 = 0x20;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_global_byte(address: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(address).read() }
        }

        let feature_flags = read_global_byte(FEATURE_FLAGS_GLOBAL);
        let service_value = if feature_flags & 2 != 0 {
            read_global_byte(SERVICE_VALUE_GLOBAL)
        } else {
            0xff
        };
        let vtable = read_word(object + VTABLE_POINTER);
        let target = read_word(vtable + SERVICE_SLOT);
        let service_call: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let service_result = service_call(
            object,
            object_value,
            u32::from(service_value),
            FIXED_ARGUMENT_A,
            FIXED_ARGUMENT_B,
            FIXED_ARGUMENT_C,
        );
        let _: u32 =
            lf_checker_rt::callee_thiscall!(FINALIZE_CALLEE, u32, this, service_result);
        service_result
    }
});
