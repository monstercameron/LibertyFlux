// original: 0x00B23440 vehicle_apply_offset_and_dispatch

/// Adds an input displacement to the selected entity origin and dispatches
/// the resulting position through the shared physics service. The first
/// pointer-like argument selects the physics record when non-null; otherwise
/// the entity's record at `+0x38` is used. The origin is the translation at
/// matrix `+0x30` when the placement matrix at `+0x20` exists, or the vector
/// at entity offset `+0x10` otherwise. The service receives the 16-bit record
/// index, the original scalar arguments, a pointer to the three summed
/// components, a zero flag, and the exact `1.0f` constant. Component sums
/// preserve the original left-to-right IEEE-754 operand order.
lf_checker_rt::export!(thiscall, rw_b23440(
    this: u32,
    subject: u32,
    displacement: u32,
    mode: u32,
    preferred_record: u32,
) -> () {
    const PLACEMENT_MATRIX: u32 = 0x20;
    const FALLBACK_ORIGIN: u32 = 0x10;
    const PHYSICS_RECORD: u32 = 0x38;
    const MATRIX_TRANSLATION: u32 = 0x30;
    const RECORD_INDEX: u32 = 0x08;
    const SERVICE_GLOBAL: u32 = 0x12B9_C7C;
    const DISPATCH_ID: u32 = 1;
    const ONE_BITS: u32 = 0x3F80_0000;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_u32(address) })
    }

    #[inline(always)]
    fn add_in_order(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    unsafe {
        let record = if preferred_record != 0 {
            preferred_record
        } else {
            read_u32(this.wrapping_add(PHYSICS_RECORD))
        };
        if record == 0 {
            return;
        }

        let matrix = read_u32(this.wrapping_add(PLACEMENT_MATRIX));
        let origin = if matrix != 0 {
            matrix.wrapping_add(MATRIX_TRANSLATION)
        } else {
            this.wrapping_add(FALLBACK_ORIGIN)
        };
        let position = [
            add_in_order(read_f32(origin), read_f32(displacement)),
            add_in_order(read_f32(origin.wrapping_add(4)), read_f32(displacement.wrapping_add(4))),
            add_in_order(read_f32(origin.wrapping_add(8)), read_f32(displacement.wrapping_add(8))),
        ];
        let index = read_u32(record.wrapping_add(RECORD_INDEX)) & 0xFFFF;
        let service = lf_checker_rt::global::<u32>(SERVICE_GLOBAL).read_unaligned();
        let position_ptr = position.as_ptr() as u32;
        let _ = lf_checker_rt::callee_thiscall!(
            DISPATCH_ID,
            u32,
            service,
            index,
            subject,
            position_ptr,
            mode,
            0,
            ONE_BITS
        );
    }
});
