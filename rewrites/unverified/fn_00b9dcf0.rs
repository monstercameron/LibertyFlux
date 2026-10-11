// original: 0x00B9DCF0 NativeImpl_IS_ANY_CHAR_SHOOTING_IN_AREA

lf_checker_rt::export!(cdecl, rw_00b9dcf0(_engine_callback: u32, context: u32) -> u32 {
    const CONTEXT_RETURN_SLOT: u32 = 0;
    const CONTEXT_ARGUMENTS: u32 = 8;
    const AREA_FLAG_OFFSET: u32 = 0x18;
    const AREA_QUERY_CALLBACK: u32 = 1;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        (address as *const u32).read_unaligned()
    }

    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        (address as *mut u32).write_unaligned(value);
    }

    unsafe {
        let return_slot = read_word(context + CONTEXT_RETURN_SLOT);
        let arguments = read_word(context + CONTEXT_ARGUMENTS);
        let mut call_block = [0u32; 7];
        let mut index = 0;
        while index < 6 {
            call_block[index] = read_word(arguments + index as u32 * 4);
            index += 1;
        }
        call_block[6] = u32::from(read_word(arguments + AREA_FLAG_OFFSET) != 0);

        let callback_result: u8 = lf_checker_rt::callee_thiscall!(
            AREA_QUERY_CALLBACK,
            u8,
            call_block.as_mut_ptr() as u32,
            call_block[0], call_block[1], call_block[2],
            call_block[3], call_block[4], call_block[5], call_block[6],
        );
        write_word(return_slot, callback_result as u32);
    }
    0
});
