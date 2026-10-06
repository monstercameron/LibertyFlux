// original: 0x009049f0 input_slot_announce (proposed)
/// Announce a slot's payload when its flag bit is set.
///
/// `idx` selects a slot of the handle table, falling back to the static
/// default index when the kind byte at `+8` is zero. Returns the object's
/// payload address (`+0x60`) when bit 6 of the flag byte at `+0x20` is
/// clear. When the bit is set the payload is first passed (with two zeros)
/// to the formatter callee and its answer is handed to the sink callee (a
/// thiscall on the static sink object), whose answer is then returned.
/// Cdecl with one stack word.
export!(cdecl, rw_009049f0(idx: u32) -> u32 {
    unsafe {
        /// Handle table base (file VA).
        const TABLE: u32 = 0x0118F6F8;
        /// Static default-slot index word (file VA).
        const DEFAULT: u32 = 0x01034494;
        /// Static sink object for the second callee (file VA).
        const SINK: u32 = 0x0116BFF0;
        const KIND_OFF: u32 = 0x08;
        const FLAG_OFF: u32 = 0x20;
        const FLAG_BIT: u8 = 0x40;
        const PAYLOAD_OFF: u32 = 0x60;
        const FMT_ID: u32 = 1;
        const SINK_ID: u32 = 2;
        let mut obj = ((relocated(TABLE).wrapping_add(idx.wrapping_mul(4))) as *const u32)
            .read_unaligned();
        if ((obj.wrapping_add(KIND_OFF)) as *const u8).read() == 0 {
            let d = (global::<u32>(DEFAULT)).read_unaligned();
            obj = ((relocated(TABLE).wrapping_add(d.wrapping_mul(4))) as *const u32)
                .read_unaligned();
        }
        let payload = obj.wrapping_add(PAYLOAD_OFF);
        if ((obj.wrapping_add(FLAG_OFF)) as *const u8).read() & FLAG_BIT != 0 {
            let text: u32 = callee_cdecl!(FMT_ID, u32, payload, 0u32, 0u32);
            return callee_thiscall!(SINK_ID, u32, relocated(SINK), text);
        }
        payload
    }
});
