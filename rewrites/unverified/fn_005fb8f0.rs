// original: 0x005FB8F0 rage::fragInst::vf57

/// Return the resource word at `+0x164` when the resource at instance `+0x64`
/// and the object resolved from the type reference or fallback pointer are
/// both nonnull. Otherwise return zero. This is a thiscall getter.

lf_checker_rt::export!(thiscall, rw_005fb8f0(this: u32) -> u32 {
    const RESOURCE_POINTER: u32 = 0x64;
    const REFERENCED_TYPE: u32 = 0x5c;
    const FALLBACK_OBJECT: u32 = 0x6c;
    const TYPE_FLAGS: u32 = 0x08;
    const TYPE_MASK: u32 = 0x3000_0000;
    const RESOURCE_VALUE: u32 = 0x164;

    unsafe {
        let resource = read_u32(this.wrapping_add(RESOURCE_POINTER));
        if resource == 0 {
            return 0;
        }
        let referenced = read_u32(this.wrapping_add(REFERENCED_TYPE));
        let object = if referenced == 0 {
            read_u32(this.wrapping_add(FALLBACK_OBJECT))
        } else if read_u32(referenced.wrapping_add(TYPE_FLAGS)) & TYPE_MASK == TYPE_MASK {
            read_u32(referenced)
        } else {
            0
        };
        if object == 0 { 0 } else { read_u32(resource.wrapping_add(RESOURCE_VALUE)) }
    }
}
