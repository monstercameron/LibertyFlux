// original: 0x00A7F4D0 CVehicleTaskInfo::vf6

/// Serialize the vehicle task information to `stream`.
///
/// The method receives the task-info record in ECX and one stream pointer on
/// the stack. It first asks the base task-info serializer for a byte, then asks
/// the stream helper to fill a one-byte flag. A zero flag selects the word at
/// `this + 0x28`; a nonzero flag selects the word at `this + 0x24`. The
/// selected value, mode 5, and a byte-sized output slot are passed to the
/// stream helper. The signed result of virtual slot `+0x50` controls whether
/// the helper at the end runs: values `<= 0` stop there, while positive values
/// also fetch the value at `this + 0x2c` and pass it, the second virtual
/// result, and zero to that helper. The return keeps the final call's upper
/// 24 bits and replaces AL with the byte written by the stream helper.
///
/// All engine calls are represented by scripted checker callees. The two
/// pointers to stack scratch are passed through those callees; their addresses
/// are intentionally not compared because the Rust and original frames differ.
lf_checker_rt::export!(thiscall, rw_00A7F4D0(this: u32, stream: u32) -> u32 {
    unsafe {
        const STREAM_SLOT_BYTE: u32 = 0;
        const VEHICLE_FLAG: u32 = 0x0B;
        const FLAG_FALSE_FIELD: u32 = 0x28;
        const FLAG_TRUE_FIELD: u32 = 0x24;
        const FINAL_FIELD: u32 = 0x2C;
        const VALIDATION_SLOT: u32 = 0x50;
        const VALIDATION_MODE: u32 = 5;

        let base_result = lf_checker_rt::callee_thiscall!(1, u32, this, stream);
        let mut stream_word = (stream & !0xFF) | (base_result & 0xFF);

        let mut flag_storage = 0u32;
        let flag_byte_ptr = (&mut flag_storage as *mut u32).cast::<u8>().add(3);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, stream, flag_byte_ptr as u32);
        let flag = core::ptr::read_volatile(flag_byte_ptr);

        let selected_field_offset = if flag == 0 {
            FLAG_FALSE_FIELD
        } else {
            FLAG_TRUE_FIELD
        };
        let selected_field = core::ptr::read((this.wrapping_add(selected_field_offset)) as *const u32);
        let stream_slot = (&mut stream_word as *mut u32) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            3,
            u32,
            stream,
            selected_field,
            VALIDATION_MODE,
            stream_slot
        );

        let validation_result = lf_checker_rt::callee_thiscall!(4, u32, this);
        let final_eax = if (validation_result as i32) <= 0 {
            validation_result
        } else {
            // The original leaves the pushed zero for the final helper, whose
            // third stack argument is zero; the virtual method is cdecl.
            let second_validation = lf_checker_rt::callee_thiscall!(4, u32, this);
            let final_field = core::ptr::read((this.wrapping_add(FINAL_FIELD)) as *const u32);
            lf_checker_rt::callee_thiscall!(5, u32, stream, final_field, second_validation, 0)
        };

        let output_byte = core::ptr::read_volatile(
            (stream_slot.wrapping_add(STREAM_SLOT_BYTE)) as *const u8
        );
        (final_eax & 0xFFFF_FF00) | output_byte as u32
    }
});
