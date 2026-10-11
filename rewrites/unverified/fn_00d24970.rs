// original: 0x00D24970 expand_bound_ranges
/// Expands a vehicle model's stored float bounds using one indexed global handling-data pointer.
/// The signed 16-bit model index selects the data record, whose three byte flags govern two pairs
/// of current and threshold floats; comparisons choose the second operand on ties and unordered
/// values, as the original ordered float comparisons do. The thiscall method takes no stack
/// arguments and writes only the selected bound pair.
lf_checker_rt::export!(thiscall, rw_00d24970(this: u32) -> () {
    const MODEL_POINTER: u32 = 0;
    const MODEL_INDEX: u32 = 0x2e;
    const TARGET_LIST: u32 = 0x34;
    const TARGET_ENTRY: u32 = 4;
    const HANDLING_TABLE_VA: u32 = 0x01295cd8;
    const FLAG_3C1: u32 = 0x3c1;
    const FLAG_3C2: u32 = 0x3c2;
    const FLAG_3C3: u32 = 0x3c3;
    const PRIMARY_LOW: u32 = 0x9c;
    const PRIMARY_HIGH: u32 = 0xa0;
    const SECONDARY_LOW: u32 = 0xb4;
    const SECONDARY_HIGH: u32 = 0xb8;
    const TERTIARY_LOW: u32 = 0xac;
    const TERTIARY_HIGH: u32 = 0xb0;
    unsafe fn read_u32(base: u32, offset: u32) -> u32 {
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    }
    unsafe fn read_u16(base: u32, offset: u32) -> u16 {
        unsafe { (base.wrapping_add(offset) as *const u16).read_unaligned() }
    }
    unsafe fn read_u8(base: u32, offset: u32) -> u8 {
        unsafe { (base.wrapping_add(offset) as *const u8).read() }
    }
    unsafe fn read_f32(base: u32, offset: u32) -> f32 {
        f32::from_bits(read_u32(base, offset))
    }
    unsafe fn write_f32(base: u32, offset: u32, value: f32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value.to_bits()) }
    }
    fn greater_or_right(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        if left > right { left } else { right }
    }
    unsafe {
        let model = read_u32(this, MODEL_POINTER);
        let model_index = read_u16(model, MODEL_INDEX) as i16;
        let handling_table = lf_checker_rt::global::<u32>(HANDLING_TABLE_VA);
        let handling_data = handling_table.wrapping_offset(model_index as isize).read_unaligned();
        let target_list = read_u32(model, TARGET_LIST);
        let target_data = read_u32(target_list, TARGET_ENTRY);
        let flag_3c1 = read_u8(handling_data, FLAG_3C1);
        let flag_3c2 = read_u8(handling_data, FLAG_3C2);
        let flag_3c3 = read_u8(handling_data, FLAG_3C3);
        let mut low = 0.0f32;
        let mut high = 0.0f32;
        if flag_3c1 == 0 {
            low = read_f32(target_data, PRIMARY_LOW);
            high = read_f32(target_data, PRIMARY_HIGH);
        }
        if flag_3c3 == 0 {
            low = greater_or_right(low, read_f32(target_data, SECONDARY_LOW));
            high = greater_or_right(high, read_f32(target_data, SECONDARY_HIGH));
        }
        if flag_3c2 == 0 {
            low = greater_or_right(low, read_f32(target_data, TERTIARY_LOW));
            high = greater_or_right(high, read_f32(target_data, TERTIARY_HIGH));
        }
        if flag_3c1 != 0 || (flag_3c3 == 0 && flag_3c2 == 0) {
            let current_low = read_f32(target_data, PRIMARY_LOW);
            let current_high = read_f32(target_data, PRIMARY_HIGH);
            write_f32(target_data, PRIMARY_LOW, greater_or_right(current_low, low));
            write_f32(target_data, PRIMARY_HIGH, greater_or_right(current_high, high));
        } else if flag_3c1 == 0 && flag_3c3 != 0 {
            let current_low = read_f32(target_data, SECONDARY_LOW);
            let current_high = read_f32(target_data, SECONDARY_HIGH);
            write_f32(target_data, SECONDARY_LOW, greater_or_right(current_low, low));
            write_f32(target_data, SECONDARY_HIGH, greater_or_right(current_high, high));
        }
    }
});
