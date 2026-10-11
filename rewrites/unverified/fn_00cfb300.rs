// original: 0x00cfb300 select_task_slot_and_forward
/// Reads the selected task index at offset 0x2B0. If the corresponding three-word
/// table row's first word is zero, forwards index zero; otherwise forwards the index.
/// It calls the target with the subobject at +0x2B0 and arguments 0, index, 1, 0.
lf_checker_rt::export!(stdcall, rw_00cfb300(input: *mut u8) -> u32 {
    unsafe {
        const INDEX_FIELD: usize = 0x2B0;
        const ROWS: usize = 0x2D4;
        let index = *((input.add(INDEX_FIELD)) as *const u32);
        let cell = *((input.add(ROWS + index as usize * 12)) as *const u32);
        let forwarded_index = if cell == 0 { 0 } else { index };
        lf_checker_rt::callee_thiscall!(1, u32, input.add(INDEX_FIELD) as u32, 0, forwarded_index, 1, 0)
    }
});
