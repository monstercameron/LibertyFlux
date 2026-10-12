// original: 0x005C36C0 CPedFactoryNY::vf2

/// Creates or retrieves a factory object from the three incoming stack words.
/// A non-null result is queried through virtual slot +0xd0; three nonzero
/// scripted values feed the registration helper as the object, the third
/// value plus 0x80, and the second value. The method then compares a scripted
/// single-precision result with the relocated zero constant. Only an exact
/// zero, including negative zero, triggers the final virtual method; a
/// nonzero low-byte result clears object byte +0x63. The object pointer is
/// returned in EAX.
lf_checker_rt::export!(thiscall, rw_005c36c0(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const OBJECT_VTABLE: u32 = 0x00;
        const QUERY_SLOT: u32 = 0xd0;
        const FLOAT_SOURCE_FILE_VA: u32 = 0x0128e310;
        const ZERO_CONSTANT_FILE_VA: u32 = 0x00fe8628;
        const STATE_BYTE: u32 = 0x63;
        const BASE_CALLEE: u32 = 1;
        const REGISTER_CALLEE: u32 = 3;
        const FLOAT_CALLEE: u32 = 4;
        const FINAL_CALLEE: u32 = 5;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_byte(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn call_thiscall0(target: u32, object: u32) -> u32 {
            let method: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            method(object)
        }

        let object = lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, this, arg0, arg1, arg2);
        if object != 0 {
            let first_vtable = read_word(object + OBJECT_VTABLE);
            let first_target = read_word(first_vtable + QUERY_SLOT);
            let first_value = call_thiscall0(first_target, object);
            if first_value != 0 {
                let second_vtable = read_word(object + OBJECT_VTABLE);
                let second_target = read_word(second_vtable + QUERY_SLOT);
                let second_value = call_thiscall0(second_target, object);

                let third_vtable = read_word(object + OBJECT_VTABLE);
                let third_target = read_word(third_vtable + QUERY_SLOT);
                let third_value = call_thiscall0(third_target, object);
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    REGISTER_CALLEE,
                    u32,
                    object,
                    third_value.wrapping_add(0x80),
                    second_value,
                    0,
                    0,
                    0,
                );
            }
        }

        let float_source = lf_checker_rt::relocated(FLOAT_SOURCE_FILE_VA);
        let result: f32 = lf_checker_rt::callee_thiscall!(FLOAT_CALLEE, f32, float_source);
        let zero_bits = lf_checker_rt::global::<u32>(ZERO_CONSTANT_FILE_VA).read_unaligned();
        let exact_zero = f32::from_bits(zero_bits);
        if object != 0 && result == exact_zero {
            let final_status: u32 =
                lf_checker_rt::callee_thiscall!(FINAL_CALLEE, u32, object, 0);
            if final_status as u8 != 0 {
                write_byte(object + STATE_BYTE, 0);
            }
        }
        object
    }
});
