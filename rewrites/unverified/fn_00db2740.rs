// original: 0x00DB2740 UITexture::bind_texture_resource

/// Bind a texture resource unless the receiver's virtual readiness query is
/// already true. The four stack arguments are a signed resource index, a
/// rectangle value, a pointer to a word copied into +0x1E0, and a signed
/// optional selection index. The method records an allocated four-byte slot
/// at +0x1D4, calls the rectangle helper only for a nonnegative first index,
/// sets the virtual resource state with argument one, writes the 16-bit mode
/// at +0x230, and stores the final index at +0x258 when nonnegative. If the
/// readiness query is already true, the method returns immediately with that
/// byte in AL and leaves all resource fields untouched.
lf_checker_rt::export!(thiscall, rw_00db2740(
    this: u32,
    resource_index: u32,
    rectangle_value: u32,
    descriptor: u32,
    selected_index: u32,
) -> u32 {
    const VTABLE_READY_SLOT: u32 = 0x140;
    const VTABLE_SET_SLOT: u32 = 0x13C;
    const RESOURCE_SLOT: u32 = 0x1D4;
    const DESCRIPTOR_COPY: u32 = 0x1E0;
    const RESOURCE_MODE: u32 = 0x230;
    const SELECTED_INDEX: u32 = 0x258;

    unsafe {
        let vtable = (this as *const u32).read_unaligned();
        let ready_address = (vtable.wrapping_add(VTABLE_READY_SLOT) as *const u32).read_unaligned();
        let is_ready: extern "thiscall" fn(u32) -> u8 =
            core::mem::transmute(ready_address as usize);
        let readiness = is_ready(this);
        if readiness != 0 {
            return readiness as u32;
        }

        let allocated = lf_checker_rt::callee_cdecl!(3, u32, 4);
        if allocated != 0 {
            (allocated as *mut u32).write_unaligned(0);
        }
        (this.wrapping_add(RESOURCE_SLOT) as *mut u32).write_unaligned(allocated);

        if (resource_index as i32) > -1 {
            let _ = lf_checker_rt::callee_thiscall!(
                4, u32, this, resource_index, rectangle_value
            );
        }

        let descriptor_word = (descriptor as *const u32).read_unaligned();
        (this.wrapping_add(DESCRIPTOR_COPY) as *mut u32).write_unaligned(descriptor_word);

        let vtable = (this as *const u32).read_unaligned();
        let set_address = (vtable.wrapping_add(VTABLE_SET_SLOT) as *const u32).read_unaligned();
        let set_resource_state: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(set_address as usize);
        let _ = set_resource_state(this, 1);

        (this.wrapping_add(RESOURCE_MODE) as *mut u16).write_unaligned(0x0100);
        if (selected_index as i32) > -1 {
            (this.wrapping_add(SELECTED_INDEX) as *mut u32).write_unaligned(selected_index);
        }
    }
    selected_index
});
