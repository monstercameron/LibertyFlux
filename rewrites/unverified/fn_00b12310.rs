// original: 0x00B12310 heli_release_active_resource

/// Notify the attachment manager when the resource at offset 0x78 has its low
/// flag bit set. Then call the virtual resource lookup. A non-null result is
/// looked up again and its virtual method at +0xe0 supplies the object forwarded
/// to the tail helper. When the first lookup is null, forward the fallback at
/// +0x100 instead. The virtual and tail calls use the planted function-table
/// stubs. This is a no-argument thiscall method.
lf_checker_rt::export!(thiscall, rw_00b12310(this: u32) -> u32 {
    const RESOURCE: u32 = 0x78;
    const FALLBACK: u32 = 0x100;
    const MANAGER_VA: u32 = 0x0166_9D78;

    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn call_lookup(this: u32) -> u32 {
        let vtable = read_u32(this);
        let target = read_u32(vtable.wrapping_add(0xA0));
        let call: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        call(this)
    }
    unsafe fn call_resource_method(resource: u32) -> u32 {
        let vtable = read_u32(resource);
        let target = read_u32(vtable.wrapping_add(0xE0));
        let call: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        call(resource)
    }

    unsafe {
        let resource = read_u32(this.wrapping_add(RESOURCE));
        if resource != 0 && read_u8(resource) & 1 != 0 {
            let manager = lf_checker_rt::global::<u8>(MANAGER_VA) as u32;
            let _ = lf_checker_rt::callee_thiscall!(
                1,
                u32,
                manager,
                resource.wrapping_add(0x1A04),
                1u32
            );
        }

        let first_result = call_lookup(this);
        if first_result == 0 {
            return lf_checker_rt::callee_thiscall!(
                4,
                u32,
                read_u32(this.wrapping_add(FALLBACK))
            );
        }

        let second_result = call_lookup(this);
        let selected = call_resource_method(second_result);
        lf_checker_rt::callee_thiscall!(4, u32, selected)
    }
});
