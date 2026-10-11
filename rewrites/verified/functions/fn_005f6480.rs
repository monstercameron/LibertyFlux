// original: 0x005F6480 rage::fragInstNM::vf9

/// Return immediately when the resource-ready flag is clear. Otherwise return
/// zero if the resource pointer is null; for a live resource, resolve the
/// optional referenced type and pass its selected word to the resource check.
///
/// The instance stores its status word at `0x60`, its referenced type at
/// `0x5c`, its resource at `0x64`, and a fallback word at `0x6c`. A referenced
/// type is accepted only when its flags at `+0x08` contain both bits in
/// `0x30000000`; the helper receives the type's first word when accepted,
/// zero when rejected, or the fallback word when there is no referenced type.
/// The helper's low-byte result is returned. This is a thiscall method with no
/// stack arguments.
lf_checker_rt::export!(thiscall, rw_005f6480(this: u32) -> u8 {
    const INSTANCE_STATUS: u32 = 0x60;
    const REFERENCED_OBJECT: u32 = 0x5c;
    const RESOURCE_POINTER: u32 = 0x64;
    const FALLBACK_ARGUMENT: u32 = 0x6c;
    const RESOURCE_READY_BIT: u32 = 0x02;
    const TYPE_FLAGS: u32 = 0x08;
    const TYPE_MASK: u32 = 0x3000_0000;
    const RESOURCE_CHECK: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        if read_u32(this.wrapping_add(INSTANCE_STATUS)) & RESOURCE_READY_BIT == 0 {
            return 1;
        }

        let resource = read_u32(this.wrapping_add(RESOURCE_POINTER));
        if resource == 0 {
            return 0;
        }

        let referenced = read_u32(this.wrapping_add(REFERENCED_OBJECT));
        let argument = if referenced == 0 {
            read_u32(this.wrapping_add(FALLBACK_ARGUMENT))
        } else if read_u32(referenced.wrapping_add(TYPE_FLAGS)) & TYPE_MASK == TYPE_MASK {
            read_u32(referenced)
        } else {
            0
        };
        lf_checker_rt::callee_thiscall!(RESOURCE_CHECK, u8, resource, argument)
    }
});
