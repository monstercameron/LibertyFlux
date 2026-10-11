// original: 0x00B12550 heli_update_component_blend

/// Skip the component update when the entity's blend bit is already set or
/// its encoded state is 3 or 6. Otherwise test the three early virtual hooks in
/// order and continue when any reports true. Resolve the virtual resource or
/// fallback, then require a live physics resource and its update flag. Three
/// further virtual hooks choose either the default blend or zero; a shared
/// override flag can replace that value with a second global float. Forward
/// the exact float bits and entity to the physics updater. Function-table
/// entries and the direct helper are checker stand-ins. This is a no-argument
/// thiscall method.
lf_checker_rt::export!(thiscall, rw_00b12550(this: u32) -> () {
    const FLAGS: u32 = 0xF4;
    const STATE: u32 = 0x28;
    const PHYSICS: u32 = 0x78;
    const FALLBACK: u32 = 0x100;
    const OVERRIDE_FLAG_VA: u32 = 0x0116_0C39;
    const DEFAULT_BLEND_VA: u32 = 0x0117_35BC;
    const OVERRIDE_BLEND_VA: u32 = 0x0117_359C;

    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn call_virtual(this: u32, slot: u32) -> u8 {
        let vtable = read_u32(this);
        let target = read_u32(vtable.wrapping_add(slot));
        let call: extern "thiscall" fn(u32) -> u8 =
            core::mem::transmute(target as usize);
        call(this)
    }
    unsafe fn call_virtual_pointer(this: u32, slot: u32) -> u32 {
        let vtable = read_u32(this);
        let target = read_u32(vtable.wrapping_add(slot));
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
    unsafe fn read_global_f32(address: u32) -> f32 {
        unsafe { (lf_checker_rt::global::<f32>(address) as *const f32).read_unaligned() }
    }

    unsafe {
        let flags = read_u8(this.wrapping_add(FLAGS));
        let state = (read_u32(this.wrapping_add(STATE)) >> 6) & 0xF;
        if flags & 4 == 0 && state != 3 && state != 6 {
            if call_virtual(this, 0x24) == 0
                && call_virtual(this, 0x28) == 0
                && call_virtual(this, 0x2C) == 0
            {
                return;
            }
        }

        let first_lookup = call_virtual_pointer(this, 0xA0);
        let resource_result = if first_lookup != 0 {
            let second_lookup = call_virtual_pointer(this, 0xA0);
            call_resource_method(second_lookup)
        } else {
            read_u32(this.wrapping_add(FALLBACK))
        };
        if resource_result == 0
            || read_u32(this.wrapping_add(PHYSICS)) == 0
            || read_u8(this.wrapping_add(FLAGS)) & 1 == 0
        {
            return;
        }

        let mut blend = if call_virtual(this, 0x24) != 0
            || call_virtual(this, 0x2C) != 0
            || call_virtual(this, 0x28) != 0
        {
            0.0f32
        } else {
            read_global_f32(DEFAULT_BLEND_VA)
        };
        let use_override =
            (lf_checker_rt::global::<u8>(OVERRIDE_FLAG_VA) as *const u8).read() != 0;
        if use_override {
            blend = read_global_f32(OVERRIDE_BLEND_VA);
        }

        let physics = read_u32(this.wrapping_add(PHYSICS));
        let _ = lf_checker_rt::callee_thiscall!(
            6,
            u32,
            physics,
            this,
            blend.to_bits()
        );
    }
});
