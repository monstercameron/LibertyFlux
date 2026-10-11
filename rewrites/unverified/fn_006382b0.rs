// original: 0x006382B0 rage::ptxEffectRuleStd::vf32

/// Query all eligible effect records and return the maximum reported value.
///
/// The thiscall receiver stores a pointer array at `+0x130` and an unsigned
/// 16-bit count at `+0x134`. Records with a nonzero word at `+0x14` are
/// skipped. For each remaining record, a cdecl helper receives the record,
/// two zero arguments, and two relocated name pointers. Its result points to
/// an object whose `+0x4c` field is an optional instance. When present, the
/// instance's virtual method at slot `+0x84` is called with no stack args.
/// A result exactly equal to the global `-1.0` sentinel returns that value
/// immediately. Otherwise only an ordered value strictly above the running
/// `+0.0` maximum replaces it; unordered values and signed zero do not.
/// The result is returned in x87 ST0. The 16-bit count is zero-extended, and
/// array address arithmetic wraps at 32 bits like the original ABI.
lf_checker_rt::export!(thiscall, rw_006382B0(this: u32) -> f32 {
    const EFFECT_RECORDS: u32 = 0x130;
    const EFFECT_COUNT: u32 = 0x134;
    const RECORD_SKIP_WORD: u32 = 0x14;
    const QUERY_OBJECT_RESULT: u32 = 0x4C;
    const VTABLE_MAXIMUM_SLOT: u32 = 0x84;
    const QUERY_HELPER: u32 = 1;
    const HELPER_NAME_A: u32 = 0x0115_0ACC;
    const HELPER_NAME_B: u32 = 0x0115_0A60;
    const SENTINEL_FILE_VA: u32 = 0x00FE_8D94;

    unsafe {
        let record_array = (this.wrapping_add(EFFECT_RECORDS) as *const u32).read_unaligned();
        let count = (this.wrapping_add(EFFECT_COUNT) as *const u16).read_unaligned() as u32;
        let sentinel = lf_checker_rt::global::<f32>(SENTINEL_FILE_VA).read_unaligned();
        let mut maximum = 0.0_f32;
        let mut index = 0_u32;

        while index < count {
            let record = (record_array.wrapping_add(index.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            index = index.wrapping_add(1);
            if (record.wrapping_add(RECORD_SKIP_WORD) as *const u32).read_unaligned() != 0 {
                continue;
            }

            let query = lf_checker_rt::callee_cdecl!(
                QUERY_HELPER,
                u32,
                record,
                0_u32,
                lf_checker_rt::relocated(HELPER_NAME_A),
                lf_checker_rt::relocated(HELPER_NAME_B),
                0_u32
            );
            let instance = (query.wrapping_add(QUERY_OBJECT_RESULT) as *const u32).read_unaligned();
            if instance == 0 {
                continue;
            }

            let vtable = (instance as *const u32).read_unaligned();
            let method_address = (vtable.wrapping_add(VTABLE_MAXIMUM_SLOT) as *const u32)
                .read_unaligned();
            let method: unsafe extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(method_address as usize);
            let candidate = method(instance);

            if candidate == sentinel {
                return sentinel;
            }
            if candidate > maximum {
                maximum = candidate;
            }
        }

        maximum
    }
});
