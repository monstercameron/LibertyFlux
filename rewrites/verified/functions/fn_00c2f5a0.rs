// original: 0x00C2F5A0 NativeImpl_SET_CAR_DOOR_LATCHED_3

/// Update the door-latch level and flags. The method saves the previous level
/// from byte offset `0x14` at `0x18`, stores the supplied level at `0x14`, and
/// conditionally updates the current value at `0x1c`. Object flag bit 18 keeps
/// that value unchanged; otherwise argument bit `0x2000` enables a limit to
/// within half of the mutable global step value. The method merges supplied flags after
/// clearing the old bits 8 through 15, then copies the new level to `0x18` when
/// any bit in `0x0c00` is set. This thiscall method takes an `f32` level and a
/// 32-bit flag word and has no semantic return value.
lf_checker_rt::export!(thiscall, rw_00c2f5a0(
    this: *mut u8,
    level_bits: u32,
    supplied_flags: u32,
) -> () {
    const FLAGS_OFFSET: usize = 0x10;
    const LEVEL_OFFSET: usize = 0x14;
    const PREVIOUS_LEVEL_OFFSET: usize = 0x18;
    const CURRENT_VALUE_OFFSET: usize = 0x1c;
    const RAMP_STEP_GLOBAL_VA: u32 = 0x011735bc;
    const HALF_STEP: f32 = 0.5;
    const CLAMP_DISABLED: u32 = 1 << 18;
    const CLAMP_REQUESTED: u32 = 0x2000;
    const FEEDBACK_MASK: u32 = 0x0c00;
    const CLEAR_REPLACED_FLAG_BITS: u32 = 0xffff00ff;

    unsafe {
        let old_flags = this.add(FLAGS_OFFSET).cast::<u32>().read_unaligned();
        let previous_level = this.add(LEVEL_OFFSET).cast::<u32>().read_unaligned();
        this.add(PREVIOUS_LEVEL_OFFSET).cast::<u32>().write_unaligned(previous_level);

        if old_flags & CLAMP_DISABLED == 0 {
            let supplied_level = f32::from_bits(level_bits);
            let current_level = if supplied_flags & CLAMP_REQUESTED != 0 {
                let current = this.add(CURRENT_VALUE_OFFSET).cast::<f32>().read_unaligned();
                let global_step = lf_checker_rt::global::<f32>(RAMP_STEP_GLOBAL_VA).read_unaligned();
                let half_step = core::hint::black_box(
                    core::hint::black_box(global_step) * core::hint::black_box(HALF_STEP),
                );
                let upper = core::hint::black_box(
                    core::hint::black_box(current) + core::hint::black_box(half_step),
                );
                if supplied_level > upper {
                    upper
                } else {
                    let lower = core::hint::black_box(
                        core::hint::black_box(current) - core::hint::black_box(half_step),
                    );
                    if lower.is_nan() || supplied_level.is_nan() || lower <= supplied_level {
                        supplied_level
                    } else {
                        lower
                    }
                }
            } else {
                supplied_level
            };
            this.add(CURRENT_VALUE_OFFSET).cast::<f32>().write_unaligned(current_level);
        }

        this.add(LEVEL_OFFSET).cast::<u32>().write_unaligned(level_bits);
        let new_flags = (old_flags & CLEAR_REPLACED_FLAG_BITS) | supplied_flags;
        this.add(FLAGS_OFFSET).cast::<u32>().write_unaligned(new_flags);
        if new_flags & FEEDBACK_MASK != 0 {
            this.add(PREVIOUS_LEVEL_OFFSET).cast::<u32>().write_unaligned(level_bits);
        }
    }
});
