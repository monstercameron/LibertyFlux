// original: 0x0096A410 find_subway_sample

/// Looks up a container through the fixed registry using `resource_key`, then
/// scans its byte-counted 0x33-byte rows for `row_key`. When a container is
/// found, `index_out` is set to zero before the scan and to the matching row
/// number on a hit. The container pointer is returned, including on a miss.
/// The stdcall entry consumes all three stack arguments.
lf_checker_rt::export!(stdcall, rw_0096a410(resource_key: u32, row_key: u32, index_out: u32) -> u32 {
    const REGISTRY: u32 = 0x0115d9a0;
    const ROW_COUNT: u32 = 0x0a;
    const FIRST_ROW_KEY: u32 = 0x0b;
    const ROW_STRIDE: u32 = 0x33;

    let container = lf_checker_rt::callee_thiscall!(
        1,
        u32,
        lf_checker_rt::relocated(REGISTRY),
        resource_key,
    );
    if container == 0 {
        return 0;
    }

    unsafe {
        (index_out as *mut u8).write(0);
        let row_count = (container.wrapping_add(ROW_COUNT) as *const u8).read();
        for row_index in 0..u32::from(row_count) {
            let key_address = container
                .wrapping_add(FIRST_ROW_KEY)
                .wrapping_add(row_index.wrapping_mul(ROW_STRIDE));
            let key = (key_address as *const u32).read_unaligned();
            if key == row_key {
                (index_out as *mut u8).write(row_index as u8);
                break;
            }
        }
    }
    container
});
