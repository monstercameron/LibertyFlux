// original: 0x009042d0 input_slot_notify_kind (proposed)
/// Notify the sink matching a slot's device kind.
///
/// `idx` selects a slot; a null slot returns 0. The device kind at `+0x48`
/// comes from the slot, or from the static default slot when the kind byte
/// at `+8` is zero. Kind 1 notifies the first sink, 2 the second, 3 the
/// third (each a thiscall on its static object with the slot's word at
/// `+0x24`, or -1 for a defaulted slot), returning its answer; any other
/// kind returns 0. Cdecl with one stack word.
export!(cdecl, rw_009042d0(idx: u32) -> u32 {
    unsafe {
        /// Handle table base (file VA).
        const TABLE: u32 = 0x0118F6F8;
        /// Static default-slot index word (file VA).
        const DEFAULT: u32 = 0x01034494;
        /// Static words holding the sink objects for kinds 1, 2, 3 (file VAs
        /// of the words; their values are passed as `this`).
        const SINK1: u32 = 0x012E22A4;
        const SINK2: u32 = 0x018B6F1C;
        const SINK3: u32 = 0x01632C60;
        const KIND_OFF: u32 = 0x08;
        const VAL_OFF: u32 = 0x24;
        const DEV_OFF: u32 = 0x48;
        const NOTIFY_ID: u32 = 1;
        let obj = ((relocated(TABLE).wrapping_add(idx.wrapping_mul(4))) as *const u32)
            .read_unaligned();
        if obj == 0 {
            return 0;
        }
        let direct = ((obj.wrapping_add(KIND_OFF)) as *const u8).read() != 0;
        let src = if direct {
            obj
        } else {
            let d = (global::<u32>(DEFAULT)).read_unaligned();
            ((relocated(TABLE).wrapping_add(d.wrapping_mul(4))) as *const u32).read_unaligned()
        };
        let kind = ((src.wrapping_add(DEV_OFF)) as *const u32).read_unaligned();
        let arg = if direct {
            ((obj.wrapping_add(VAL_OFF)) as *const u32).read_unaligned()
        } else {
            0xFFFFFFFF
        };
        let sinkw = if kind == 1 {
            SINK1
        } else if kind == 2 {
            SINK2
        } else if kind == 3 {
            SINK3
        } else {
            return 0;
        };
        let sink = (global::<u32>(sinkw)).read_unaligned();
        callee_thiscall!(NOTIFY_ID, u32, sink, arg)
    }
});
