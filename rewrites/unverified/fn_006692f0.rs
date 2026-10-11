// original: 0x006692F0 rage::ptxEventEffect::vf12

/// Clear the event pointer field on the second stack argument and return that
/// record pointer. The x86 `thiscall` receiver in ECX and the first stack word
/// are unused. The record field is a 32-bit value at byte offset `0x110`;
/// there is no null check, so an invalid record pointer faults at the store.
const EVENT_POINTER_FIELD_OFFSET: u32 = 0x110;

lf_checker_rt::export!(thiscall, rw_006692f0(_receiver: u32, _unused: u32, event_record: u32) -> u32 {
    unsafe {
        let field = event_record.wrapping_add(EVENT_POINTER_FIELD_OFFSET) as *mut u32;
        field.write_unaligned(0);
    }
    event_record
});
