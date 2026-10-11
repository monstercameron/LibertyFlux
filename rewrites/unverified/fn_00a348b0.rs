// original: 0x00A348B0 get_world_object_value

/// Reads a type-specific value from a world object. A null object returns
/// zero. Type 3 selects the fallback field at +0x7b0, or uses +0x7b4 when the
/// selector has bit 6 set and that field is nonzero. Type 2 dispatches through
/// vtable slot +0xa0 with the object in ECX. Other types read the field at
/// +0x38. This is a cdecl function with an object pointer and selector word.
lf_checker_rt::export!(cdecl, rw_00a348b0(object: u32, selector: u32) -> u32 {
    const TYPE_FLAGS: u32 = 0x28;
    const DEFAULT_VALUE: u32 = 0x7b0;
    const SELECTED_VALUE: u32 = 0x7b4;
    const OTHER_VALUE: u32 = 0x38;
    const VTABLE_SLOT: u32 = 0xa0;
    const TYPE_SHIFT: u32 = 6;
    const TYPE_MASK: u32 = 0x0f;
    const SELECTOR_BIT: u32 = 0x40;

    if object == 0 {
        return 0;
    }

    unsafe {
        let flags = ((object + TYPE_FLAGS) as *const u32).read_unaligned();
        let object_type = (flags >> TYPE_SHIFT) & TYPE_MASK;
        if object_type == 3 {
            if selector & SELECTOR_BIT != 0 {
                let selected = ((object + SELECTED_VALUE) as *const u32).read_unaligned();
                if selected != 0 {
                    return selected;
                }
            }
            return ((object + DEFAULT_VALUE) as *const u32).read_unaligned();
        }

        if object_type == 2 {
            let vtable = (object as *const u32).read_unaligned();
            let method_address = ((vtable + VTABLE_SLOT) as *const u32).read_unaligned();
            let method: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(method_address as usize);
            return method(object);
        }

        ((object + OTHER_VALUE) as *const u32).read_unaligned()
    }
});
