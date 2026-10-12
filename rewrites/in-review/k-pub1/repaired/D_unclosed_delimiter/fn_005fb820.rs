// original: 0x005FB820 rage::fragInst::vf55

/// Resolve the object from the referenced type or fallback pointer. If it is
/// nonnull and its float at `+0x200` differs from the global zero value,
/// clear bit 1 in the instance status at `+0x60` unless the object byte at
/// `+0x1f7` already has bit 1 set. IEEE NaN compares unequal to zero, while
/// either signed zero compares equal. The method has no return value.

lf_checker_rt::export!(thiscall, rw_005fb820(this: u32) -> () {
    const REFERENCED_TYPE: u32 = 0x5c;
    const INSTANCE_STATUS: u32 = 0x60;
    const FALLBACK_OBJECT: u32 = 0x6c;
    const TYPE_FLAGS: u32 = 0x08;
    const TYPE_MASK: u32 = 0x3000_0000;
    const FLOAT_PROPERTY: u32 = 0x200;
    const OBJECT_FLAGS: u32 = 0x1f7;
    const READY_BIT: u32 = 0x02;
    const ZERO_FLOAT_GLOBAL: u32 = 0x00fe_8628;

    unsafe {
        let referenced = read_u32(this.wrapping_add(REFERENCED_TYPE));
        let object = if referenced == 0 {
            read_u32(this.wrapping_add(FALLBACK_OBJECT))
        } else if read_u32(referenced.wrapping_add(TYPE_FLAGS)) & TYPE_MASK == TYPE_MASK {
            read_u32(referenced)
        } else {
            0
        };
        if object == 0 {
            return;
        }

        let value = read_f32(object.wrapping_add(FLOAT_PROPERTY));
        let zero = core::ptr::read_unaligned(lf_checker_rt::global::<f32>(ZERO_FLOAT_GLOBAL));
        if value != zero {
            let status_object = if referenced == 0 {
                read_u32(this.wrapping_add(FALLBACK_OBJECT))
            } else if read_u32(referenced.wrapping_add(TYPE_FLAGS)) & TYPE_MASK == TYPE_MASK {
                read_u32(referenced)
            } else {
                0
            };
            if (status_object.wrapping_add(OBJECT_FLAGS) as *const u8).read_unaligned() & READY_BIT == 0 {
                let status_address = this.wrapping_add(INSTANCE_STATUS);
                let status = read_u32(status_address);
                write_u32(status_address, status & !READY_BIT);
            }
        }
    }
});

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

#[inline(always)]
unsafe fn read_f32(address: u32) -> f32 {
    f32::from_bits(unsafe { read_u32(address) })
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}
