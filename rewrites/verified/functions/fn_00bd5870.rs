// original: 0x00bd5870 NativeImpl_GET_PHYSICAL_SCREEN_RESOLUTION

/// Write the active display width and height through the two output pointers. When the mode-table flag is clear, select the 16-byte row returned by the display-mode helper; otherwise choose each fallback dimension from its boolean system-query result. Return the final height word on the table path and the height output pointer on the fallback path.
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn rw_00bd5870(
    width_output: u32,
    height_output: u32,
) -> u32 {
    const MODE_TABLE_FLAG: u32 = 0x017a_ccf8;
    const MODE_TABLE_POINTER: u32 = 0x0116_8bb0;
    const FALLBACK_WIDTH: u32 = 0x0105_c884;
    const FALLBACK_WIDTH_ALT: u32 = 0x0105_c888;
    const FALLBACK_HEIGHT: u32 = 0x0105_c880;
    const FALLBACK_HEIGHT_ALT: u32 = 0x0105_c87c;
    const CURRENT_MODE: u32 = 1;
    const SYSTEM_QUERY: u32 = 2;

    unsafe {
        let mode_table_flag = lf_checker_rt::global::<u8>(MODE_TABLE_FLAG).read_unaligned();
        if mode_table_flag == 0 {
            let mode_index = lf_checker_rt::callee_cdecl!(CURRENT_MODE, u32,);
            let table_address = lf_checker_rt::global::<u32>(MODE_TABLE_POINTER).read_unaligned();
            let row = (table_address as usize + (mode_index.wrapping_sub(1) as usize) * 16) as *const u32;
            let width = row.read_unaligned();
            let height = (row as *const u8).add(4).cast::<u32>().read_unaligned();
            (width_output as *mut u32).write_unaligned(width);
            (height_output as *mut u32).write_unaligned(height);
            height
        } else {
            let width_alt = lf_checker_rt::callee_cdecl!(SYSTEM_QUERY, u32,);
            let width = if width_alt & 0xff == 0 {
                lf_checker_rt::global::<u32>(FALLBACK_WIDTH).read_unaligned()
            } else {
                lf_checker_rt::global::<u32>(FALLBACK_WIDTH_ALT).read_unaligned()
            };
            (width_output as *mut u32).write_unaligned(width);

            let height_alt = lf_checker_rt::callee_cdecl!(SYSTEM_QUERY, u32,);
            let height = if height_alt & 0xff == 0 {
                lf_checker_rt::global::<u32>(FALLBACK_HEIGHT).read_unaligned()
            } else {
                lf_checker_rt::global::<u32>(FALLBACK_HEIGHT_ALT).read_unaligned()
            };
            (height_output as *mut u32).write_unaligned(height);
            height_output
        }
    }
}
