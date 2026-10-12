// original: 0x00880410 destroy_utf16_object_array

/// Destroy each element in a UTF-16 object array, release the array storage,
/// then destroy the base subobject.
///
/// The receiver stores the array pointer at byte offset `0x58` and a
/// zero-extended 16-bit element count at `0x5e`. For a zero count, the array
/// pointer is not read and the element destructor and storage release are
/// skipped. Otherwise, call the element destructor once per element at a
/// 100-byte stride, release the original array pointer, and tail-call the
/// base destructor with `this + 0x0c`. The count is unsigned; all loop
/// iterations use the same call order as the original thiscall routine.
lf_checker_rt::export!(thiscall, rw_00880410(this: u32) -> u32 {
    unsafe {
        const ARRAY_POINTER_OFFSET: u32 = 0x58;
        const ELEMENT_COUNT_OFFSET: u32 = 0x5E;
        const BASE_SUBOBJECT_OFFSET: u32 = 0x0C;
        const ELEMENT_STRIDE: u32 = 0x64;
        const DESTROY_ELEMENT_CALLEE: u32 = 1;
        const RELEASE_ARRAY_CALLEE: u32 = 2;
        const DESTROY_BASE_CALLEE: u32 = 3;

        let count = (this.wrapping_add(ELEMENT_COUNT_OFFSET) as *const u16)
            .read_unaligned() as u32;
        if count != 0 {
            let array_base = (this.wrapping_add(ARRAY_POINTER_OFFSET) as *const u32)
                .read_unaligned();
            let mut element = array_base;
            for _ in 0..count {
                let _ = lf_checker_rt::callee_thiscall!(
                    DESTROY_ELEMENT_CALLEE, u32, element
                );
                element = element.wrapping_add(ELEMENT_STRIDE);
            }
            let _ = lf_checker_rt::callee_cdecl!(RELEASE_ARRAY_CALLEE, u32, array_base);
        }

        lf_checker_rt::callee_thiscall!(
            DESTROY_BASE_CALLEE,
            u32,
            this.wrapping_add(BASE_SUBOBJECT_OFFSET)
        )
    }
});
