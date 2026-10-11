// original: 0x005FCD30 streaming_sum_item_values

/// Sum the single precision value at offset `+4` in each referenced item.
///
/// The receiver stores an item count as a byte at `+0x1f3` and a pointer to an
/// array of item pointers at `+0xd4`. Values are read in array order, added as
/// `running_sum + item_value` with one single precision rounding per item, and
/// returned in ST(0). An empty array returns positive zero. The function uses
/// the 32-bit thiscall receiver and takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_005fcd30(this: u32) -> f32 {
    unsafe {
        const ITEM_ARRAY: u32 = 0xd4;
        const ITEM_COUNT: u32 = 0x1f3;
        const ITEM_VALUE: u32 = 4;

        let count = (this.wrapping_add(ITEM_COUNT) as *const u8).read() as u32;
        let items = (this.wrapping_add(ITEM_ARRAY) as *const u32).read_unaligned();
        let mut running_sum = 0.0f32;
        for index in 0..count {
            let item = (items.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
            let bits = (item.wrapping_add(ITEM_VALUE) as *const u32).read_unaligned();
            let item_value = f32::from_bits(bits);
            running_sum = core::hint::black_box(running_sum) + core::hint::black_box(item_value);
        }
        running_sum
    }
});
