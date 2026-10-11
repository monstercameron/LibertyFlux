// original: 0x005FDFC0 streaming_update_item_capacity

/// Update the byte capacity and, when it grows, notify the TLS-backed service.
///
/// The receiver stores its current capacity at `+0x1f1`, the first service
/// result at `+0xfc`, the second at `+0x100`, and the capacity output at
/// `+0x1f0`. The input is the low byte of the single stack argument. Values
/// less than or equal to the current capacity only update `+0x1f0` and return
/// unchanged. A larger value is rounded up to the next power-of-two capacity
/// (with byte wrapping at 256), then sent twice through the TLS slot-zero
/// service's virtual method at vtable offset `+8`. The two returned words are
/// passed to the notification helper with their prior receiver fields, then
/// stored back at `+0xfc` and `+0x100`. The rounded byte replaces `+0x1f1`,
/// while the requested byte is written to `+0x1f0` and returned. This uses
/// thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_005fdfc0(this: u32, input_word: u32) -> u8 {
    unsafe {
        const CURRENT_CAPACITY: u32 = 0x1f1;
        const CAPACITY_OUTPUT: u32 = 0x1f0;
        const FIRST_SERVICE_RESULT: u32 = 0xfc;
        const SECOND_SERVICE_RESULT: u32 = 0x100;
        const SERVICE_POINTER_IN_TLS_ROOT: u32 = 8;
        const SERVICE_METHOD: u32 = 8;
        const NOTIFY_ID: u32 = 2;
        const NOTIFY_ARGUMENT: u32 = 0x10;

        let requested = input_word as u8;
        let current = (this.wrapping_add(CURRENT_CAPACITY) as *const u8).read();
        if requested <= current {
            (this.wrapping_add(CAPACITY_OUTPUT) as *mut u8).write(requested);
            return requested;
        }

        let mut spread = requested;
        spread |= spread >> 1;
        spread |= spread >> 2;
        spread |= spread >> 4;
        let new_capacity = spread.wrapping_add(1);

        let tls_root = lf_checker_rt::tls_slot(0);
        let service = (tls_root.wrapping_add(SERVICE_POINTER_IN_TLS_ROOT) as *const u32)
            .read_unaligned();
        let vtable = (service as *const u32).read_unaligned();
        let method_address =
            (vtable.wrapping_add(SERVICE_METHOD) as *const u32).read_unaligned();
        let service_method: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(method_address as usize);

        let first_result = service_method(service, new_capacity as u32, NOTIFY_ARGUMENT, 0);
        let second_result = service_method(service, new_capacity as u32, NOTIFY_ARGUMENT, 0);
        let old_requested =
            (this.wrapping_add(CAPACITY_OUTPUT) as *const u8).read() as u32;
        let old_first =
            (this.wrapping_add(FIRST_SERVICE_RESULT) as *const u32).read_unaligned();
        let old_second =
            (this.wrapping_add(SECOND_SERVICE_RESULT) as *const u32).read_unaligned();
        let _ = lf_checker_rt::callee_cdecl!(
            NOTIFY_ID,
            u32,
            first_result,
            old_first,
            old_requested,
        );
        let _ = lf_checker_rt::callee_cdecl!(
            NOTIFY_ID,
            u32,
            second_result,
            old_second,
            old_requested,
        );

        (this.wrapping_add(SECOND_SERVICE_RESULT) as *mut u32)
            .write_unaligned(second_result);
        (this.wrapping_add(FIRST_SERVICE_RESULT) as *mut u32).write_unaligned(first_result);
        (this.wrapping_add(CURRENT_CAPACITY) as *mut u8).write(new_capacity);
        (this.wrapping_add(CAPACITY_OUTPUT) as *mut u8).write(requested);
        requested
    }
});
