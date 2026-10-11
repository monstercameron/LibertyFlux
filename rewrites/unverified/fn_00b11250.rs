// original: 0x00B11250 CHeli::vf22

/// Select the virtual resource result, falling back to the object's field at
/// offset 0x38. A live resource supplies the value through the nested pointer
/// chain at +4, +0xc and +8; when both pointers are null, use the indexed global
/// record's value at +0x1c. Add the object's float at +0xfc in the original
/// operand order and return the exact single-precision result in ST0.
lf_checker_rt::export!(thiscall, rw_00b11250(this: u32) -> f32 {
    #[inline(always)]
    fn add_in_order(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn read_f32(address: u32) -> f32 {
        unsafe { (address as *const f32).read_unaligned() }
    }

    unsafe {
        let lookup = lf_checker_rt::callee_thiscall!(1, u32, this);
        let resource = if lookup != 0 {
            lookup
        } else {
            read_u32(this.wrapping_add(0x38))
        };
        let source_value = if resource != 0 {
            let first_link = read_u32(resource.wrapping_add(4));
            let second_link = read_u32(first_link.wrapping_add(0xC));
            read_f32(second_link.wrapping_add(8))
        } else {
            const TABLE_VA: u32 = 0x0129_5CD8;
            let index = (this.wrapping_add(0x2E) as *const i16).read_unaligned() as i32;
            let table = lf_checker_rt::global::<u32>(TABLE_VA) as u32;
            let item = read_u32(table.wrapping_add((index as u32).wrapping_mul(4)));
            read_f32(item.wrapping_add(0x1C))
        };
        add_in_order(source_value, read_f32(this.wrapping_add(0xFC)))
    }
});
