// original: 0x005FD890 streaming_resolve_item_parameters

/// Resolve one streaming item and forward its parameter block to the helper.
///
/// The receiver contains a context pointer at `+0xb4`, an indexed item table
/// at `+0xd0`, and a category dispatch table at `+0xd4`. The context's data
/// descriptor is at `+0x0c` and contains a base pointer at offset zero; the
/// selected item contributes a category byte at
/// `+0x10e`, and the selected table contributes a signed 16-bit row at
/// `+0x0e`. A missing data base returns zero. Otherwise the row offset is
/// scaled by 224 and passed in ECX, the address of the original index value
/// is passed in EDX, and five temporary output-byte pointers are passed on
/// the stack. The helper's EAX result is returned. The outer function uses
/// thiscall and cleans its one input word itself.
lf_checker_rt::export!(thiscall, rw_005fd890(this: u32, item_index: u32) -> u32 {
    unsafe {
        const CONTEXT_POINTER: u32 = 0xb4;
        const ITEM_TABLE_POINTER: u32 = 0xd0;
        const CATEGORY_TABLE_POINTER: u32 = 0xd4;
        const CONTEXT_DATA_BASE: u32 = 0x0c;
        const ITEM_CATEGORY: u32 = 0x10e;
        const TABLE_ROW: u32 = 0x0e;
        const ROW_STRIDE: i32 = 224;
        const HELPER_ID: u32 = 1;

        let mut item_index = item_index;
        let context = (this.wrapping_add(CONTEXT_POINTER) as *const u32).read_unaligned();
        let data_descriptor =
            (context.wrapping_add(CONTEXT_DATA_BASE) as *const u32).read_unaligned();
        if data_descriptor == 0 {
            return 0;
        }
        let data_base = (data_descriptor as *const u32).read_unaligned();

        let item_table = (this.wrapping_add(ITEM_TABLE_POINTER) as *const u32).read_unaligned();
        let item = (item_table.wrapping_add(item_index.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let category = (item.wrapping_add(ITEM_CATEGORY) as *const u8).read() as u32;
        let category_table =
            (this.wrapping_add(CATEGORY_TABLE_POINTER) as *const u32).read_unaligned();
        let row_table = (category_table.wrapping_add(category.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let row = (row_table.wrapping_add(TABLE_ROW) as *const i16).read_unaligned() as i32;
        let helper_receiver = data_base.wrapping_add(row.wrapping_mul(ROW_STRIDE) as u32);

        let index_pointer = (&mut item_index as *mut u32) as u32;
        let mut output_bytes = [0u8; 8];
        let result = lf_checker_rt::callee_fastcall!(
            HELPER_ID,
            u32,
            helper_receiver,
            index_pointer,
            (&mut output_bytes[0] as *mut u8) as u32,
            (&mut output_bytes[1] as *mut u8) as u32,
            (&mut output_bytes[2] as *mut u8) as u32,
            (&mut output_bytes[3] as *mut u8) as u32,
            (&mut output_bytes[4] as *mut u8) as u32,
        );
        result
    }
});
