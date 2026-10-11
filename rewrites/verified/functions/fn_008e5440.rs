// original: 0x008E5440 audio_state_index_4a0_setter

/// Store an audio-state table index and cache its mapped value.
///
/// The one stack argument is an unsigned table index. The method stores it at
/// byte offset `0x4a0`, reads the corresponding dword from the
/// static table beginning at `0x01032FAC`, stores that value at byte offset
/// `0x48c`, and returns the loaded dword in EAX. It uses the
/// 32-bit thiscall convention and removes its one stack word with the callee pops 4 bytes.
lf_checker_rt::export!(thiscall, rw_008e5440(this: u32, table_index: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x0103_2fac;
        const INDEX_FIELD: usize = 0x4a0;
        const VALUE_FIELD: usize = 0x48c;

        let index_field = (this as usize as *mut u32).add(INDEX_FIELD / 4);
        index_field.write_unaligned(table_index);
        let table_value = (lf_checker_rt::global::<u32>(TABLE_BASE)
            .add(table_index as usize))
            .read_unaligned();
        (this as usize as *mut u32).add(VALUE_FIELD / 4).write_unaligned(table_value);
        table_value
    }
});
