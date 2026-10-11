// original: 0x00B11AA0 heli_update_resource_handles

/// When the physics resource has its low flag bit set, notify the shared
/// attachment manager with the resource's embedded block and mode one. Then
/// call the object's virtual resource lookup. A non-null result is looked up a
/// second time and its slot at +0xe0 is called; otherwise the fallback field at
/// +0x100 is used and the virtual lookup is retried. The selected value is
/// forwarded to the tail helper, while a null fallback returns zero. The
/// vtable calls and tail helper are scripted stand-ins. This is a no-argument
/// thiscall method.
lf_checker_rt::export!(thiscall, rw_00b11aa0(this: u32) -> u32 {
    const RESOURCE: u32 = 0x78;
    const FALLBACK: u32 = 0x100;
    const RESOURCE_FLAG: u8 = 1;
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
        if resource != 0 && read_u8(resource) & RESOURCE_FLAG != 0 {
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
        let selected = if first_result != 0 {
            let next = call_lookup(this);
            call_resource_method(next)
        } else {
            let fallback = read_u32(this.wrapping_add(FALLBACK));
            if fallback == 0 {
                return 0;
            }
            let retried = call_lookup(this);
            if retried == 0 {
                return lf_checker_rt::callee_thiscall!(4, u32, fallback);
            }
            let next = call_lookup(this);
            call_resource_method(next)
        };

        lf_checker_rt::callee_thiscall!(4, u32, selected)
    }
});
