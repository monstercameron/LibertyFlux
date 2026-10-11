// original: 0x005F9E40 rage::fragInst::vf29

/// Resolve the active fragment object, run its selected resource helpers,
/// optionally populate and process an output object, set the ready bit, and
/// return the instance or zero on an early failure.
///
/// The referenced type pointer is at `+0x5c`, its flags are at `+0x08`, and
/// the fallback object pointer is at `+0x6c`. The instance status word is at
/// `+0x60`; bit `0x02` gates entry and bit `0x10` suppresses resource
/// initialization. An accepted object has bit zero set in byte `+0x1f7`.
/// The resource pointer is at `+0x64`. It may expose a gate object at `+0x19c`,
/// a status word at `+0x174`, and primary or fallback output pointers at
/// `+0x17c` and `+0x180`. The optional output argument points to a word that
/// receives the selected object. Its nested object at `+0x18` gets width and
/// height words computed by wrapping subtraction of two, then an active byte
/// cleared to zero. The method returns `this` after processing and zero on
/// its early failure paths. All helper calls use the seeded stand-ins in the
/// contract; frame and heap memory remain directly compared.
lf_checker_rt::export!(thiscall, rw_005f9e40(this: u32, output_pointer: u32) -> u32 {
    const REFERENCED_TYPE: u32 = 0x5c;
    const INSTANCE_STATUS: u32 = 0x60;
    const RESOURCE_POINTER: u32 = 0x64;
    const FALLBACK_OBJECT: u32 = 0x6c;
    const TYPE_FLAGS: u32 = 0x08;
    const TYPE_MASK: u32 = 0x3000_0000;
    const READY_BIT: u32 = 0x02;
    const RESOURCE_ENABLED_BIT: u32 = 0x10;
    const OBJECT_READY: u32 = 0x1f7;
    const RESOURCE_STATUS: u32 = 0x174;
    const RESOURCE_PRIMARY: u32 = 0x17c;
    const RESOURCE_FALLBACK: u32 = 0x180;
    const RESOURCE_GATE: u32 = 0x19c;
    const OBJECT_NESTED: u32 = 0x18;
    const INNER_FIRST: u32 = 0x10;
    const INNER_SECOND: u32 = 0x14;
    const INNER_WIDTH: u32 = 0x20;
    const INNER_HEIGHT: u32 = 0x24;
    const INNER_ACTIVE: u32 = 0x28;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let referenced = read_u32(this.wrapping_add(REFERENCED_TYPE));
        let object = if referenced == 0 {
            read_u32(this.wrapping_add(FALLBACK_OBJECT))
        } else if read_u32(referenced.wrapping_add(TYPE_FLAGS)) & TYPE_MASK == TYPE_MASK {
            read_u32(referenced)
        } else {
            0
        };

        let status = read_u32(this.wrapping_add(INSTANCE_STATUS));
        if status & READY_BIT == 0 {
            return 0;
        }
        if read_u8(object.wrapping_add(OBJECT_READY)) & 1 == 0 {
            write_u32(this.wrapping_add(INSTANCE_STATUS), status | READY_BIT);
            return this;
        }

        let resource = read_u32(this.wrapping_add(RESOURCE_POINTER));
        if resource == 0 {
            let ready = lf_checker_rt::callee_thiscall!(2, u32, this);
            if ready == 0 {
                return 0;
            }
        }
        if status & RESOURCE_ENABLED_BIT == 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, resource);
        }

        let gate = read_u32(resource.wrapping_add(RESOURCE_GATE));
        if gate != 0 && lf_checker_rt::callee_thiscall!(3, u8, gate) == 0 {
            return 0;
        }

        if output_pointer != 0 {
            let mut candidate = read_u32(resource.wrapping_add(RESOURCE_PRIMARY));
            if candidate == 0 {
                candidate = read_u32(resource.wrapping_add(RESOURCE_FALLBACK));
            }
            if candidate != 0 {
                write_u32(output_pointer, candidate);
            }

            let first_object = read_u32(output_pointer);
            if first_object != 0 {
                let _ = lf_checker_rt::callee_thiscall!(4, u32, first_object);
                let second_object = read_u32(output_pointer);
                let _ = lf_checker_rt::callee_thiscall!(5, u32, second_object);

                let inner = read_u32(second_object.wrapping_add(OBJECT_NESTED));
                if inner != 0 {
                    let first_extent = read_u32(inner.wrapping_add(INNER_FIRST));
                    let second_extent = read_u32(inner.wrapping_add(INNER_SECOND));
                    write_u32(inner.wrapping_add(INNER_WIDTH), first_extent.wrapping_sub(2));
                    write_u32(inner.wrapping_add(INNER_HEIGHT), second_extent.wrapping_sub(2));
                    write_u8(inner.wrapping_add(INNER_ACTIVE), 0);
                    let _ = lf_checker_rt::callee_thiscall!(6, u32, inner);
                }
            }
        }

        if read_u32(resource.wrapping_add(RESOURCE_STATUS)) != 0 {
            let _ = lf_checker_rt::callee_thiscall!(7, u32, this);
        }
        write_u32(this.wrapping_add(INSTANCE_STATUS), status | READY_BIT);
        this
    }
});
