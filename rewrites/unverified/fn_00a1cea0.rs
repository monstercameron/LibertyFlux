// original: 0x00A1CEA0 ped_task_recalculate_blend_totals

/// Recalculate three blend totals on a task-like object.
///
/// The object holds four signed record counts at offsets `+0x400` through
/// `+0x40c`, four byte enable arrays beginning at `+0x410`, and the output
/// triplet at `+0x420`. Counts are tested as signed values; zero and negative
/// counts visit no records. Enabled records in the first and third groups
/// contribute the component-wise average of two triplets, multiplied by
/// one half. Enabled records in the second and fourth groups contribute one
/// triplet directly. Every enabled record increments one shared signed count.
/// If at least one record was enabled, each accumulated component is divided
/// by that count; otherwise the three outputs remain zero. EAX returns the
/// third group's final loop index unless the fourth group's count is
/// positive, in which case it returns that group's end cursor. The object uses a
/// 32-bit game layout;
/// its offsets are named here because the merged layout does not identify
/// this larger task record.
///
/// Floating-point operations keep the original order: pairwise additions,
/// multiplication by one half, accumulation, reciprocal division, then
/// multiplication of each total by the reciprocal. The calling convention is
/// thiscall with the object in ECX and no stack arguments.
lf_checker_rt::export!(thiscall, rw_00A1CEA0(this: u32) -> u32 {
    unsafe {
        const GROUP_COUNTS: [u32; 4] = [0x400, 0x404, 0x408, 0x40c];
        const GROUP_FLAGS: [u32; 4] = [0x410, 0x411, 0x412, 0x413];
        const GROUP_BASES: [u32; 4] = [0x108, 0x1d8, 0x2a8, 0x3a8];
        const GROUP_STRIDES: [u32; 4] = [0xb0, 0xc0, 0xe0, 0x120];
        const OUTPUTS: u32 = 0x420;
        const COMPONENTS: u32 = 3;
        const HALF: f32 = 0.5;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_i32(address: u32) -> i32 {
            unsafe { read_u32(address) as i32 }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            unsafe { f32::from_bits(read_u32(address)) }
        }
        #[inline(always)]
        unsafe fn write_f32(address: u32, value: f32) {
            unsafe { (address as *mut u32).write_unaligned(value.to_bits()) }
        }
        #[inline(always)]
        fn add_ss(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        fn mul_ss(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        fn div_ss(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) / core::hint::black_box(right)
        }

        let object = this;
        let mut component = 0;
        while component < COMPONENTS {
            write_f32(object + OUTPUTS + component * 4, 0.0);
            component += 1;
        }

        let mut enabled_count = 0i32;
        let mut return_eax = 0u32;
        let mut group = 0usize;
        while group < GROUP_COUNTS.len() {
            let count = read_i32(object + GROUP_COUNTS[group]);
            let mut index = 0i32;
            if count > 0 {
                while index < count {
                    let record_flag = GROUP_FLAGS[group] + index as u32;
                    if read_u8(object + record_flag) != 0 {
                        enabled_count = enabled_count.wrapping_add(1);
                        let record = object
                            .wrapping_add(GROUP_BASES[group])
                            .wrapping_add((index as u32).wrapping_mul(GROUP_STRIDES[group]));
                        component = 0;
                        while component < COMPONENTS {
                            let output = object + OUTPUTS + component * 4;
                            let accumulated = read_f32(output);
                            let contribution = if group == 0 || group == 2 {
                                let first = read_f32(record + component * 4 - 0x18);
                                let second = read_f32(record + component * 4 - 8);
                                mul_ss(add_ss(first, second), HALF)
                            } else {
                                read_f32(record + component * 4 - 8)
                            };
                            write_f32(output, add_ss(contribution, accumulated));
                            component += 1;
                        }
                    }
                    index = index.wrapping_add(1);
                }
            }
            if group == 2 {
                return_eax = index as u32;
            } else if group == 3 && count > 0 {
                return_eax = object
                    .wrapping_add(GROUP_BASES[group])
                    .wrapping_add((index as u32).wrapping_mul(GROUP_STRIDES[group]));
            }
            group += 1;
        }

        let reciprocal = if enabled_count > 0 {
            div_ss(1.0, enabled_count as f32)
        } else {
            1.0
        };
        component = 0;
        while component < COMPONENTS {
            let output = object + OUTPUTS + component * 4;
            let total = read_f32(output);
            write_f32(output, mul_ss(total, reciprocal));
            component += 1;
        }
        return_eax
    }
});
