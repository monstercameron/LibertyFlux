// original: 0x00B10F60 CHeli::vf21

/// Write a three-component affine transform of the position at the helper's
/// returned resource into the stack-supplied output buffer. Each component is
/// accumulated in the original order: one scaled source coordinate, then the
/// next two scaled coordinates, then the translation. When both the virtual
/// lookup and the fallback field are null, forward `this` and the output
/// pointer to the helper instead. Float operands are kept in source order for
/// bit-exact results. This is a thiscall method with one stack argument.
lf_checker_rt::export!(thiscall, rw_00b10f60(this: u32, output: u32) -> () {
    #[inline(always)]
    fn multiply_in_order(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
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
    unsafe fn write_f32(address: u32, value: f32) {
        unsafe { (address as *mut f32).write_unaligned(value) }
    }

    const FALLBACK: u32 = 0x38;

    unsafe {
        let lookup = lf_checker_rt::callee_thiscall!(1, u32, this);
        let resource = if lookup != 0 {
            lookup
        } else {
            read_u32(this.wrapping_add(FALLBACK))
        };
        if resource == 0 {
            let _ = lf_checker_rt::callee_thiscall!(2, u32, this, output);
            return;
        }

        let transform_root = read_u32(resource.wrapping_add(4));
        let transform = read_u32(transform_root.wrapping_add(0xC));
        let scale_x = read_f32(transform.wrapping_add(0x30));
        let scale_y = read_f32(transform.wrapping_add(0x34));
        let scale_z = read_f32(transform.wrapping_add(0x38));

        let first = add_in_order(
            add_in_order(
                multiply_in_order(read_f32(resource.wrapping_add(0x20)), scale_y),
                multiply_in_order(read_f32(resource.wrapping_add(0x10)), scale_x),
            ),
            multiply_in_order(read_f32(resource.wrapping_add(0x30)), scale_z),
        );
        let first = add_in_order(first, read_f32(resource.wrapping_add(0x40)));

        let second = add_in_order(
            add_in_order(
                multiply_in_order(read_f32(resource.wrapping_add(0x24)), scale_y),
                multiply_in_order(read_f32(resource.wrapping_add(0x14)), scale_x),
            ),
            multiply_in_order(read_f32(resource.wrapping_add(0x34)), scale_z),
        );
        let second = add_in_order(second, read_f32(resource.wrapping_add(0x44)));

        let third = add_in_order(
            add_in_order(
                multiply_in_order(read_f32(resource.wrapping_add(0x28)), scale_y),
                multiply_in_order(read_f32(resource.wrapping_add(0x18)), scale_x),
            ),
            multiply_in_order(read_f32(resource.wrapping_add(0x38)), scale_z),
        );
        let third = add_in_order(third, read_f32(resource.wrapping_add(0x48)));

        write_f32(output, first);
        write_f32(output.wrapping_add(4), second);
        write_f32(output.wrapping_add(8), third);
    }
});
