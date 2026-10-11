// original: 0x00D17B10 ped_task_list_dispatch
/// Dispatch enabled entries in four embedded task lists on a task-list owner.
/// The signed counts at owner offsets `0x13a0`, `0x13a4`, `0x13a8`, and
/// `0x13ac` bound lists starting at `0x90`, `0x820`, `0x8e0`, and `0x1280`.
/// Their entry strides are `0xb0`, `0xc0`, `0xe0`, and `0x120`; enable bytes
/// begin at `0x13b0`, `0x13bb`, `0x13bc`, and `0x13c7`. All four lists
/// forward the three inputs in order. The owner arrives in ECX and the three
/// stack arguments are removed by thiscall cleanup.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00d17b10(owner: u32, first: u32, second: u32, third: u32) {
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    unsafe fn run_list(
        owner: u32,
        count_offset: u32,
        entries_offset: u32,
        stride: u32,
        enabled_offset: u32,
        first_argument: u32,
        second_argument: u32,
        third_argument: u32,
    ) {
        let count = unsafe { read_u32(owner + count_offset) as i32 };
        let mut index = 0i32;
        while index < count {
            let item_index = index as u32;
            if unsafe { read_u8(owner + enabled_offset + item_index) } != 0 {
                let entry = owner + entries_offset + item_index * stride;
                let _ = lf_checker_rt::callee_thiscall!(
                    1,
                    u32,
                    entry,
                    first_argument,
                    second_argument,
                    third_argument,
                );
            }
            index += 1;
        }
    }

    unsafe {
        run_list(owner, 0x13a0, 0x90, 0xb0, 0x13b0, first, second, third);
        run_list(owner, 0x13a4, 0x820, 0xc0, 0x13bb, first, second, third);
        run_list(owner, 0x13a8, 0x8e0, 0xe0, 0x13bc, first, second, third);
        run_list(owner, 0x13ac, 0x1280, 0x120, 0x13c7, first, second, third);
    }
}
