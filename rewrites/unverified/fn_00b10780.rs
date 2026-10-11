// original: 0x00B10780 heli_rebuild_attached_resource

/// Use the object's virtual resource lookup, then validate the nested source
/// pointer chain and read its 16-bit subtype at offset 0x14. Allocate a
/// 0x28-byte resource, optionally construct it, and pass the resource and
/// source through the two setup helpers. When the object's array at +0x20 is
/// null, refresh its inline storage twice around the update helper. Store the
/// resulting resource, link it to the array pointer, and set flag bit 1. The
/// object vtable slot and each helper are scripted stand-ins; this method is
/// a no-argument thiscall.
lf_checker_rt::export!(thiscall, rw_00b10780(this: u32) -> () {
    const SOURCE: u32 = 0x34;
    const ARRAY: u32 = 0x20;
    const RESOURCE: u32 = 0x100;
    const FLAGS: u32 = 0xF4;

    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let lookup = lf_checker_rt::callee_thiscall!(1, u32, this);
        if lookup != 0 {
            return;
        }

        let source_root = read_u32(this.wrapping_add(SOURCE));
        if source_root == 0 {
            return;
        }
        let source_table = read_u32(source_root);
        if source_table == 0 {
            return;
        }
        let source = read_u32(source_table.wrapping_add(0xC));
        if source == 0 {
            return;
        }
        let subtype = (source.wrapping_add(0x14) as *const u16).read_unaligned() as u32;

        let allocation = lf_checker_rt::callee_cdecl!(2, u32, 0x28u32);
        let constructed = if allocation != 0 {
            lf_checker_rt::callee_thiscall!(3, u32, allocation, subtype, 1u32)
        } else {
            0
        };
        (this.wrapping_add(RESOURCE) as *mut u32).write_unaligned(constructed);
        let _ = lf_checker_rt::callee_thiscall!(4, u32, constructed, source, 0u32, 0u32);
        let array = read_u32(this.wrapping_add(ARRAY));

        if array == 0 {
            let _ = lf_checker_rt::callee_thiscall!(5, u32, this);
            let _ = lf_checker_rt::callee_thiscall!(6, u32, this.wrapping_add(0x10), array);
        }
        let _ = lf_checker_rt::callee_thiscall!(7, u32, this);
        if read_u32(this.wrapping_add(ARRAY)) == 0 {
            let _ = lf_checker_rt::callee_thiscall!(5, u32, this);
            let updated_array = read_u32(this.wrapping_add(ARRAY));
            let _ = lf_checker_rt::callee_thiscall!(6, u32, this.wrapping_add(0x10), updated_array);
        }

        let final_array = read_u32(this.wrapping_add(ARRAY));
        let resource = read_u32(this.wrapping_add(RESOURCE));
        (resource.wrapping_add(8) as *mut u32).write_unaligned(final_array);
        let flags = (this.wrapping_add(FLAGS) as *const u8).read();
        (this.wrapping_add(FLAGS) as *mut u8).write(flags | 2);
    }
});
