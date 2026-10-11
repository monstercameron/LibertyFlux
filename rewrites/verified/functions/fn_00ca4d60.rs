// original: 0x00ca4d60 CEventHandler::CEventHandler

/// Initializes an event handler around one context word. The embedded handler
/// at `+0x34` is constructed through its intercepted thiscall, the five event
/// slots at `+0x20..+0x34` and the five leading context slots are cleared,
/// and the outer and embedded vtable pointers are installed. Returns `this`.
///
/// The pointer width and object offsets follow the original 32-bit layout.
lf_checker_rt::export!(thiscall, rw_00ca4d60(this: u32, context: u32) -> u32 {
    unsafe {
        const OUTER_VTABLE: u32 = 0x00ed_7b54;
        const EMBEDDED_HANDLER: u32 = 0x34;
        const FIRST_EVENT_SLOT: u32 = 0x20;
        const EVENT_SLOT_COUNT: u32 = 5;
        const CONTEXT_SLOT_COUNT: u32 = 5;
        const NESTED_HANDLER_CTOR: u32 = 1;

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        write_u32(this, lf_checker_rt::relocated(OUTER_VTABLE));
        write_u32(this + 4, context);
        let mut offset = FIRST_EVENT_SLOT;
        for _ in 0..EVENT_SLOT_COUNT {
            write_u32(this + offset, 0);
            offset += 4;
        }
        lf_checker_rt::callee_thiscall!(
            NESTED_HANDLER_CTOR,
            u32,
            this + EMBEDDED_HANDLER,
            context
        );
        let mut context_offset = 8;
        for _ in 0..CONTEXT_SLOT_COUNT {
            write_u32(this + context_offset, 0);
            context_offset += 4;
        }
        this
    }
});
