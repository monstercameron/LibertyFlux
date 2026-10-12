// original: 0x00B7ECB0 check_drive_target_proximity

/// Checks whether a vehicle task's target point is within the fixed squared
/// distance threshold, then asks two task-owned regions whether either one
/// contains a matching entity. The receiver stores its reference position at
/// `+0x18..+0x20`; the source task points to a position record at `+0x20` and
/// to its component block at `+0x224`. The position is three `f32` values at
/// `+0x30..+0x38`. The helper calls receive the receiver, source task, a
/// component subobject, and radius 16. The method is thiscall with one stack
/// pointer and returns its boolean in AL. Squared distance is accumulated in
/// x, y, z order; unordered float comparisons follow the original's `jbe`.
lf_checker_rt::export!(thiscall, rw_b7ecb0(this: u32, source: u32) -> u32 {
    unsafe {
        const RECEIVER_POSITION: u32 = 0x18;
        const SOURCE_POSITION: u32 = 0x20;
        const SOURCE_COMPONENTS: u32 = 0x224;
        const POSITION_VALUES: u32 = 0x30;
        const COMPONENT_A: u32 = 0x168;
        const COMPONENT_B: u32 = 0x10c;
        const SEARCH_RADIUS: u32 = 16;
        const DISTANCE_LIMIT_VA: u32 = 0x00fe_8b58;
        const FIRST_QUERY: u32 = 1;
        const SECOND_QUERY: u32 = 2;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            unsafe { f32::from_bits(read_u32(address)) }
        }

        #[inline(always)]
        fn sub(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        #[inline(always)]
        fn mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        fn add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        let position = read_u32(source.wrapping_add(SOURCE_POSITION));
        let receiver_x = read_f32(this.wrapping_add(RECEIVER_POSITION));
        let receiver_y = read_f32(this.wrapping_add(RECEIVER_POSITION + 4));
        let receiver_z = read_f32(this.wrapping_add(RECEIVER_POSITION + 8));
        let point_x = read_f32(position.wrapping_add(POSITION_VALUES));
        let point_y = read_f32(position.wrapping_add(POSITION_VALUES + 4));
        let point_z = read_f32(position.wrapping_add(POSITION_VALUES + 8));

        let dx = sub(point_x, receiver_x);
        let dy = sub(point_y, receiver_y);
        let dz = sub(point_z, receiver_z);
        let distance_squared = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        let distance_limit_squared = lf_checker_rt::global::<u32>(DISTANCE_LIMIT_VA).read();
        let distance_limit_squared = f32::from_bits(distance_limit_squared);
        if distance_squared > distance_limit_squared {
            return 0;
        }

        let components = read_u32(source.wrapping_add(SOURCE_COMPONENTS));
        let first_hit = lf_checker_rt::callee_thiscall!(
            FIRST_QUERY,
            u32,
            this,
            source,
            components.wrapping_add(COMPONENT_A),
            SEARCH_RADIUS,
        );
        if first_hit as u8 != 0 {
            return 1;
        }

        let second_hit = lf_checker_rt::callee_thiscall!(
            SECOND_QUERY,
            u32,
            this,
            source,
            components.wrapping_add(COMPONENT_B),
            SEARCH_RADIUS,
        );
        u32::from(second_hit as u8 != 0)
    }
});
