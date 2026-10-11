// original: 0x005FBB80 fragment_tls_counter_update

/// Set the object's first word to its class marker, then consult the fabricated
/// thread-local record at slot 0. Three lookup helpers write through pointers to
/// the object's words at `+4`, `+8` and `+0xc`; on a result other than `0xffffffff`,
/// a nonzero written value is passed to the corresponding update helper and its
/// return is added with 32-bit wrapping. All three words are cleared before return,
/// which is the original `this` pointer. The contract disables stack comparison
/// for the original's temporary stack scratch; its helper context is compared in
/// the outgoing call log. This is a thiscall method.

lf_checker_rt::export!(thiscall, rw_005fbb80(this: u32) -> u32 {
    const CLASS_VTABLE_VA: u32 = 0x00fe_17d4;
    const TLS_SLOT: usize = 0;
    const TLS_ITEM: u32 = 0x04;
    const FIRST_VALUE: u32 = 0x04;
    const SECOND_VALUE: u32 = 0x08;
    const THIRD_VALUE: u32 = 0x0c;

    unsafe {
        write_u32(this, lf_checker_rt::relocated(CLASS_VTABLE_VA));
        let tls_record = lf_checker_rt::tls_slot(TLS_SLOT);

        let first_thread_item = read_u32(tls_record.wrapping_add(TLS_ITEM));
        let first_context = read_u32(first_thread_item);
        let first_address = this.wrapping_add(FIRST_VALUE);
        let first_status = lf_checker_rt::callee_thiscall!(1, u32, first_context, first_address);
        if first_status == u32::MAX {
            write_u32(first_address, 0);
        } else {
            let first_value = read_u32(first_address);
            if first_value != 0 {
                let first_delta = lf_checker_rt::callee_thiscall!(2, u32, first_thread_item, first_value);
                write_u32(first_address, first_value.wrapping_add(first_delta));
            }
        }

        let tls_record = lf_checker_rt::tls_slot(TLS_SLOT);
        let second_thread_item = read_u32(tls_record.wrapping_add(TLS_ITEM));
        let second_context = read_u32(second_thread_item);
        let second_address = this.wrapping_add(SECOND_VALUE);
        let second_status = lf_checker_rt::callee_thiscall!(3, u32, second_context, second_address);
        if second_status == u32::MAX {
            write_u32(second_address, 0);
        } else {
            let second_value = read_u32(second_address);
            if second_value != 0 {
                let second_delta = lf_checker_rt::callee_thiscall!(4, u32, second_thread_item, second_value);
                write_u32(second_address, second_value.wrapping_add(second_delta));
            }
        }

        let tls_record = lf_checker_rt::tls_slot(TLS_SLOT);
        let third_thread_item = read_u32(tls_record.wrapping_add(TLS_ITEM));
        let third_context = read_u32(third_thread_item);
        let third_address = this.wrapping_add(THIRD_VALUE);
        let third_status = lf_checker_rt::callee_thiscall!(5, u32, third_context, third_address);
        if third_status == u32::MAX {
            write_u32(third_address, 0);
        } else {
            let third_value = read_u32(third_address);
            if third_value != 0 {
                let third_delta = lf_checker_rt::callee_thiscall!(6, u32, third_thread_item, third_value);
                write_u32(third_address, third_value.wrapping_add(third_delta));
            }
        }

        write_u32(this.wrapping_add(SECOND_VALUE), 0);
        write_u32(this.wrapping_add(FIRST_VALUE), 0);
        write_u32(this.wrapping_add(THIRD_VALUE), 0);
        this
    }
}
