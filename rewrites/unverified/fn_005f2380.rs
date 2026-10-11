// original: 0x005F2380 CEventHandlerNY::vf81

/// For event type `0x451`, clear the handler's active object when the event
/// marker byte at `+0x219` is nonzero, then return true. Other event types
/// return false without reading the event object. The layout uses the merged
/// `CEventHandler` active-object field at `+0x0c`; the result is observed in AL.
lf_checker_rt::export!(thiscall, rw_005f2380(this: u32, event_type: u32, event: u32) -> u32 {
    const EVENT_TYPE: u32 = 0x451;
    const EVENT_MARKER: u32 = 0x219;
    const ACTIVE_OBJECT: u32 = 0x0c;

    if event_type != EVENT_TYPE {
        return 0;
    }
    // SAFETY: the checker contract supplies a readable event object on this path.
    let marker = unsafe { (event.wrapping_add(EVENT_MARKER) as *const u8).read() };
    if marker != 0 {
        // SAFETY: the checker contract supplies a writable handler object.
        unsafe { (this.wrapping_add(ACTIVE_OBJECT) as *mut u32).write_unaligned(0) };
    }
    1
});
