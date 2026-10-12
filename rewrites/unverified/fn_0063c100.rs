// original: 0x0063C100 rage::ptxModel::vf6

/// Forward this call through the model's nested object's second virtual slot.
/// The object keeps the nested pointer at offset `0x174`; the nested object
/// begins with its vtable pointer. Pass the original `this` value in ECX and
/// return the selected method's 32-bit result.
lf_checker_rt::export!(thiscall, rw_0063c100(this: *mut u8) -> u32 {
    unsafe {
        const NESTED_OBJECT: usize = 0x174;
        const VTABLE_SLOT: usize = 1;
        let nested_address = (this.add(NESTED_OBJECT) as *const u32).read_unaligned();
        let nested = nested_address as usize as *const u32;
        let vtable_address = nested.read_unaligned();
        let vtable = vtable_address as usize as *const u32;
        let method_address = vtable.add(VTABLE_SLOT).read_unaligned();
        let method: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(method_address as usize);
        method(this as u32)
    }
});
