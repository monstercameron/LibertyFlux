// original: 0x005FC490 streaming_invoke_embedded_handler

/// Invoke the handler stored in the embedded object at `this + 0x388`.
///
/// The first argument is the object pointer in ECX. Five following words are
/// forwarded unchanged; the last word carries a single precision value's raw
/// bits. The handler receives the original object in ECX and six stack words:
/// the embedded subobject pointer (`this + 0x37c`), then those five words. The
/// handler is called through the object's data slot and its EAX result is the
/// function's EAX result. The original caller cleans the handler's six words.
lf_checker_rt::export!(thiscall, rw_005fc490(
    this: u32,
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    value_bits: u32,
) -> u32 {
    unsafe {
        const EMBEDDED_SUBOBJECT: u32 = 0x37c;
        const HANDLER_SLOT: u32 = 0x388;
        let target = ((this + HANDLER_SLOT) as *const u32).read_unaligned();
        let handler: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        handler(
            this,
            this + EMBEDDED_SUBOBJECT,
            arg0,
            arg1,
            arg2,
            arg3,
            value_bits,
        )
    }
});
