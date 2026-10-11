// original: 0x00D2F8C0 path_copy_future_nodes (proposed)

/// Copy the current and later path records into the caller's output buffer,
/// stopping at the first task-side flag word that is nonzero. The receiver
/// keeps the path pointer at 0x70, signed current index at 0x98 and slot flags
/// beginning at 0x78; bit
/// 0x80 at task offset 0xdc disables copying. The output count and reserved
/// word are cleared first, then each 16-byte record is copied to offset
/// `0x10 + count * 0x10`. The return value is the copied-record count. The
/// thiscall method has one output pointer on the stack and removes four bytes.
lf_checker_rt::export!(thiscall, rw_00D2F8C0(this: u32, output: u32) -> u32 {
    const PATH_POINTER: u32 = 0x70;
    const SKIP_FLAGS: u32 = 0x78;
    const CURRENT_INDEX: u32 = 0x98;
    const TASK_FLAGS: u32 = 0xdc;
    const DISABLE_COPY: u8 = 0x80;
    const FIRST_RECORD: u32 = 0x10;
    const RECORD_STRIDE: u32 = 0x10;

    unsafe {
        crate::write_u32(output, 0);
        crate::write_u32(output.wrapping_add(4), 0);

        let path = crate::read_u32(this.wrapping_add(PATH_POINTER));
        if path == 0 || crate::read_u8(this.wrapping_add(TASK_FLAGS)) & DISABLE_COPY != 0 {
            return 0;
        }

        let mut index = crate::read_u8(this.wrapping_add(CURRENT_INDEX)) as i8 as i32;
        let node_count = crate::read_u32(path) as i32;
        if index >= node_count {
            return 0;
        }

        let mut copied = 0u32;
        while index < node_count {
            let signed_offset = index.wrapping_mul(4) as u32;
            let skip_word = this
                .wrapping_add(SKIP_FLAGS)
                .wrapping_add(signed_offset);
            if crate::read_u16(skip_word) != 0 {
                break;
            }
            {
                let record_index = copied;
                copied = copied.wrapping_add(1);
                let source = path
                    .wrapping_add(FIRST_RECORD)
                    .wrapping_add(index.wrapping_mul(RECORD_STRIDE as i32) as u32);
                let target = output
                    .wrapping_add(FIRST_RECORD)
                    .wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
                for word in 0..4u32 {
                    crate::write_u32(
                        target.wrapping_add(word * 4),
                        crate::read_u32(source.wrapping_add(word * 4)),
                    );
                }
                crate::write_u32(output, copied);
            }
            index = index.wrapping_add(1);
        }
        copied
    }
});
