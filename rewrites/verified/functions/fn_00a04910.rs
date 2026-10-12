// original: 0x00A04910 NativeImpl_GET_OBJECT_ROTATION_VELOCITY
/// Looks up the object by handle, asks a helper for its three-component
/// velocity, then multiplies that vector by the object's 3x3 transform.
/// Each output row evaluates (column 1 * velocity 1 + column 0 * velocity 0)
/// + column 2 * velocity 2 in that single-precision operand order.
lf_checker_rt::export!(cdecl, rw_00A04910(handle: u32, out_x: u32, out_y: u32, out_z: u32) -> () {
    unsafe {
        const POOL_POINTER: u32 = 0x0163_2C60;
        const TRANSFORM_POINTER: u32 = 0x20;
        const ROW_STRIDE: u32 = 0x10;
        const VELOCITY_WORDS: usize = 3;

        #[inline(never)]
        fn ordered_mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(never)]
        fn ordered_add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        let pool = lf_checker_rt::global::<u32>(POOL_POINTER).read_volatile();
        let object = lf_checker_rt::callee_thiscall!(1, u32, pool, handle);
        let mut velocity_scratch = [0u32; VELOCITY_WORDS];
        let velocity_pointer = lf_checker_rt::callee_thiscall!(
            2, u32, object, velocity_scratch.as_mut_ptr() as u32
        );
        let velocity = velocity_pointer as *const f32;
        let velocity0 = velocity.read_unaligned();
        let velocity1 = velocity.add(1).read_unaligned();
        let velocity2 = velocity.add(2).read_unaligned();
        let matrix = (object.wrapping_add(TRANSFORM_POINTER) as *const u32).read_unaligned() as *const f32;

        let row_x = ordered_add(
            ordered_add(ordered_mul(matrix.add(4).read_unaligned(), velocity1),
                        ordered_mul(matrix.read_unaligned(), velocity0)),
            ordered_mul(matrix.add(8).read_unaligned(), velocity2),
        );
        let row_y = ordered_add(
            ordered_add(ordered_mul(matrix.add(5).read_unaligned(), velocity1),
                        ordered_mul(matrix.add(1).read_unaligned(), velocity0)),
            ordered_mul(matrix.add(9).read_unaligned(), velocity2),
        );
        let row_z = ordered_add(
            ordered_add(ordered_mul(matrix.add(6).read_unaligned(), velocity1),
                        ordered_mul(matrix.add(2).read_unaligned(), velocity0)),
            ordered_mul(matrix.add(10).read_unaligned(), velocity2),
        );
        (out_x as *mut f32).write_unaligned(row_x);
        (out_y as *mut f32).write_unaligned(row_y);
        (out_z as *mut f32).write_unaligned(row_z);
    }
});
