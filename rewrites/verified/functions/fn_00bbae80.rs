// original: 0x00BBAE80 CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>::vf1

/// Clone the move-seek task in x86 thiscall form: `this` is the source task in ECX and the result is returned in EAX.
/// The task context is passed to the allocator. When it returns null, the original leaves the destination null and faults
/// on its first field write; the rewrite preserves that fault. Otherwise the base initializer receives the task's entity,
/// calculator parameters, two float words, a zero word and the low-bit flag. The clone then receives the source fields at
/// offsets `0x78`, `0x7c`, `0x18`, `0x50`, `0x58` and `0x54`, with the float at `0x18` replaced by two helper calls.
/// Four bits in the flag byte at `0x80` are copied from the source while all other destination bits are preserved.
/// Helper answers are scripted and every call's order and arguments are compared; helper internals are outside this proof.
lf_checker_rt::export!(thiscall, rw_00bbae80(this: u32) -> u32 {
    unsafe {
        const TASK_CONTEXT_VA: u32 = 0x0167E2A0;
        const TASK_CONTEXT: u32 = 1;
        const BASE_INITIALIZER: u32 = 2;
        const FLOAT_INPUT_HELPER: u32 = 3;
        const FLOAT_RESULT_HELPER: u32 = 4;
        const TASK_FLAG_BYTE: u32 = 0x80;
        const COPY_FLAG_04: u8 = 0x04;
        const COPY_FLAG_08: u8 = 0x08;
        const COPY_FLAG_10: u8 = 0x10;
        const COPY_FLAG_40: u8 = 0x40;

        let task_context = lf_checker_rt::global::<u32>(TASK_CONTEXT_VA).read_unaligned();
        let manager = lf_checker_rt::callee_thiscall!(TASK_CONTEXT, u32, task_context);
        let clone = if manager != 0 {
            let entity = (this.wrapping_add(0x20) as *const u32).read_unaligned();
            let calculator_radius = (this.wrapping_add(0x40) as *const u32).read_unaligned();
            let calculator_angle = (this.wrapping_add(0x44) as *const u32).read_unaligned();
            let base_float_48 = (this.wrapping_add(0x48) as *const u32).read_unaligned();
            let base_float_4c = (this.wrapping_add(0x4c) as *const u32).read_unaligned();
            let source_flag = (this.wrapping_add(TASK_FLAG_BYTE) as *const u8).read() & 1;
            lf_checker_rt::callee_thiscall!(
                BASE_INITIALIZER, u32, manager,
                entity, calculator_radius, calculator_angle,
                base_float_48, base_float_4c, 0, source_flag as u32
            )
        } else {
            0
        };

        let value_78 = (this.wrapping_add(0x78) as *const u32).read_unaligned();
        (clone.wrapping_add(0x78) as *mut u32).write_unaligned(value_78);
        let value_7c = (this.wrapping_add(0x7c) as *const u32).read_unaligned();
        (clone.wrapping_add(0x7c) as *mut u32).write_unaligned(value_7c);
        let value_18 = (this.wrapping_add(0x18) as *const u32).read_unaligned();
        (clone.wrapping_add(0x18) as *mut u32).write_unaligned(value_18);

        let source_flags = (this.wrapping_add(TASK_FLAG_BYTE) as *const u8).read();
        let mut clone_flags = (clone.wrapping_add(TASK_FLAG_BYTE) as *const u8).read();
        clone_flags ^= (clone_flags ^ source_flags) & COPY_FLAG_04;
        (clone.wrapping_add(TASK_FLAG_BYTE) as *mut u8).write(clone_flags);

        let value_50 = (this.wrapping_add(0x50) as *const u32).read_unaligned();
        (clone.wrapping_add(0x50) as *mut u32).write_unaligned(value_50);
        let helper_answer = lf_checker_rt::callee_cdecl!(FLOAT_INPUT_HELPER, u32, value_18);
        let adjusted_float: f32 = lf_checker_rt::callee_cdecl!(FLOAT_RESULT_HELPER, f32, helper_answer);
        (clone.wrapping_add(0x18) as *mut u32).write_unaligned(adjusted_float.to_bits());

        let value_58 = (this.wrapping_add(0x58) as *const u32).read_unaligned();
        (clone.wrapping_add(0x58) as *mut u32).write_unaligned(value_58);
        let value_54 = (this.wrapping_add(0x54) as *const u32).read_unaligned();
        (clone.wrapping_add(0x54) as *mut u32).write_unaligned(value_54);

        clone_flags ^= (clone_flags ^ source_flags) & COPY_FLAG_08;
        (clone.wrapping_add(TASK_FLAG_BYTE) as *mut u8).write(clone_flags);
        clone_flags ^= (clone_flags ^ source_flags) & COPY_FLAG_10;
        (clone.wrapping_add(TASK_FLAG_BYTE) as *mut u8).write(clone_flags);
        clone_flags ^= (clone_flags ^ source_flags) & COPY_FLAG_40;
        (clone.wrapping_add(TASK_FLAG_BYTE) as *mut u8).write(clone_flags);
        clone
    }
});
