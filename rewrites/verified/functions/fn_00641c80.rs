// original: 0x00641C80 rage::ptxModel::vf30
/// Clear the model's pending bit and tail-dispatch through vtable slot `0x18`.
///
/// The object stores a pointer to its function table at offset zero and a
/// 32-bit status word at offset `0x11c`. The method clears status bit one,
/// then forwards the original `this` pointer to the selected no-argument
/// handler and returns that handler's EAX result.
lf_checker_rt::export!(thiscall, rw_00641c80(object: u32) -> u32 {
    const STATUS_WORD: u32 = 0x11c;
    const PENDING_BIT: u32 = 0x2;
    unsafe {
        let status = object.wrapping_add(STATUS_WORD) as *mut u32;
        status.write(status.read() & !PENDING_BIT);
        lf_checker_rt::callee_thiscall!(1, u32, object)
    }
});
