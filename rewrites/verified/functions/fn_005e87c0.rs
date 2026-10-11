// original: 0x005E87C0 CEventHandlerNY::~CEventHandlerNY__deleting
const EVENT_HANDLER_VTABLE_FILE_VA: u32 = 0x00FE1184;
const TLS_CONTEXT_OFFSET: u32 = 8;
const TLS_VTABLE_OFFSET: u32 = 0;
const TLS_DELETE_SLOT_OFFSET: u32 = 0x0c;

/// Installs the event handler's base vtable and calls its base destructor.
/// If deletion flag bit zero is set, it follows TLS slot zero to a context,
/// loads the context's function table, and calls the entry at byte offset
/// `0x0c` with the context in ECX and `this` on the stack. It returns `this`
/// in EAX and pops its single stack flag.
lf_checker_rt::export!(thiscall, rw_005e87c0(this: u32, delete_flags: u32) -> u32 {
    unsafe {
        (this as *mut u32).write_unaligned(
            lf_checker_rt::relocated(EVENT_HANDLER_VTABLE_FILE_VA)
        );
    }
    lf_checker_rt::callee_thiscall!(1, u32, this);

    if delete_flags & 1 != 0 {
        let tls_object = lf_checker_rt::tls_slot(0);
        let context = unsafe {
            (tls_object.wrapping_add(TLS_CONTEXT_OFFSET) as *const u32).read_unaligned()
        };
        let function_table = unsafe {
            (context.wrapping_add(TLS_VTABLE_OFFSET) as *const u32).read_unaligned()
        };
        let function = unsafe {
            (function_table.wrapping_add(TLS_DELETE_SLOT_OFFSET) as *const u32).read_unaligned()
        };
        let indirect: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(function as usize) };
        let _ = indirect(context, this);
    }
    this
});
