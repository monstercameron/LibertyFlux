// original: 0x00CC94C0 calculate_physics_contact_blend

/// Computes a bounded physics blend factor from a manager record. The manager
/// helper supplies a record table; `index` selects one record, its float at
/// `+0x04` is divided by a per-manager rate returned by the x87 helper and the
/// byte at manager `+0x1f3`. The ratio is clamped to the global range `[0.2,
/// 1.5]`, then blended as `(1.0 - amount) + ratio * amount`. The two helpers
/// are scripted by the proof. SSE single-precision operations keep the source
/// operand order so their result bits match the original.
lf_checker_rt::export!(thiscall, rw_00cc94c0(this: u32, index: u32, amount: f32) -> f32 {
    const MANAGER_LOOKUP: u32 = 1;
    const RATE_HELPER: u32 = 2;
    const MANAGER_TABLE: u32 = 0xd4;
    const INSTANCE_COUNT: u32 = 0x1f3;
    const RECORD_VALUE: u32 = 4;
    const MIN_RATIO_VA: u32 = 0x00fe87d0;
    const MAX_RATIO_VA: u32 = 0x00fe8960;
    const BASE_VALUE_VA: u32 = 0x00fe88e8;
    const ONE: f32 = 1.0;

    #[inline(never)]
    fn divide(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        core::hint::black_box(left / right)
    }
    #[inline(never)]
    fn subtract(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        core::hint::black_box(left - right)
    }
    #[inline(never)]
    fn multiply(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        core::hint::black_box(left * right)
    }
    #[inline(never)]
    fn add(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        core::hint::black_box(left + right)
    }

    let manager = lf_checker_rt::callee_thiscall!(MANAGER_LOOKUP, u32, this);
    let instances = unsafe { (manager.wrapping_add(INSTANCE_COUNT) as *const u8).read() };
    let rate = lf_checker_rt::callee_thiscall!(RATE_HELPER, f32, manager);
    let per_instance = divide(rate, f32::from(instances));

    let table = unsafe { (manager.wrapping_add(MANAGER_TABLE) as *const u32).read_unaligned() };
    let record = unsafe {
        (table.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    };
    let record_value = unsafe {
        (record.wrapping_add(RECORD_VALUE) as *const f32).read_unaligned()
    };
    let mut ratio = divide(record_value, per_instance);

    let minimum = unsafe { lf_checker_rt::global::<f32>(MIN_RATIO_VA).read_unaligned() };
    let maximum = unsafe { lf_checker_rt::global::<f32>(MAX_RATIO_VA).read_unaligned() };
    if minimum > ratio {
        ratio = minimum;
    }
    if ratio > maximum {
        ratio = maximum;
    }

    let base = unsafe { lf_checker_rt::global::<f32>(BASE_VALUE_VA).read_unaligned() };
    let unweighted = subtract(base, amount);
    let weighted = multiply(ratio, amount);
    add(weighted, unweighted)
});
