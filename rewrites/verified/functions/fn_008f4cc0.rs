// original: 0x008f4cc0 input_device_fields_set (proposed)

/// Broadcast one configuration word to seven fields of the device object.
///
/// `obj` is the input device object, `v` an opaque 32-bit value. The
/// function stores `v` at seven fixed offsets (two in the `0x32Ax` control
/// block, one link slot at `+0x4`, and four table/axis slots elsewhere).
/// No read-modify-write: every slot is overwritten. EAX still holds `v` at
/// return, so the contract compares the full return register.
///
/// Thiscall: object in ECX, value as one stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_008f4cc0(obj: u32, v: u32) -> u32 {
    unsafe {
        const SLOTS: [u32; 7] = [0x32a4, 0x0004, 0x07bc, 0x0f74, 0x32b0, 0x1ee4, 0x172c];
        for s in SLOTS {
            ((obj + s) as *mut u32).write_unaligned(v);
        }
        v
    }
});
