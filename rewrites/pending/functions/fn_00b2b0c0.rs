// original: 0x00B2B0C0 task-record probe chain lookup
/// Look up a vehicle-task record through the type probe chain.
///
/// Reads the link word of the given object. When the link is missing, or the
/// object reports the detached type, it falls back to the object's own status
/// query: a live object with its ready flag set yields its embedded record,
/// otherwise the lookup fails. On the main path it rejects objects whose
/// auxiliary block is flagged, then probes the pool head with a fixed
/// sequence of type ids, returning the first hit adjusted past its header;
/// when only the seventh probe hits and the confirming probe misses, it
/// returns the seventh hit's extended record instead.
export!(cdecl, rw_b2b0c0(obj: u32) -> u32 {
    const LINK_OFF: usize = 0xF50;
    const TYPE_OFF: usize = 0x1304;
    const DETACHED_TYPE: u32 = 3;
    const STATUS_SLOT: u32 = 0x144;
    const READY_OFF: usize = 0xEA0;
    const RECORD_OFF: u32 = 0xEA4;
    const AUX_OFF: usize = 0x6C;
    const AUX_FLAG_OFF: usize = 0x0E;
    const POOL_OFF: usize = 0x224;
    const HEAD_DELTA: u32 = 0x44;
    const HIT_DELTA: u32 = 0x14;
    const EXTENDED_DELTA: u32 = 0x60;
    const PROBE_IDS: [u32; 7] = [0x2D4, 0x2C7, 0x2C6, 0x2D1, 0x2D6, 0x2D9, 0x2E2];
    const CONFIRM_ID: u32 = 0x32E;
    const LAST_ID: u32 = 0x2D3;
    unsafe {
        let link = (obj as *const u32).byte_add(LINK_OFF).read();
        if link == 0 || (obj as *const u32).byte_add(TYPE_OFF).read() == DETACHED_TYPE {
            let vt = (obj as *const u32).read();
            let status: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vt.wrapping_add(STATUS_SLOT)) as *const u32).read() as usize,
            );
            if status(obj) & 0xFF == 0 {
                return 0;
            }
            if (obj as *const u8).byte_add(READY_OFF).read() == 0 {
                return 0;
            }
            return obj.wrapping_add(RECORD_OFF);
        }
        let aux = (link as *const u32).byte_add(AUX_OFF).read();
        if aux != 0 && (aux as *const u8).byte_add(AUX_FLAG_OFF).read() != 0 {
            return 0;
        }
        let pool = (link as *const u32).byte_add(POOL_OFF).read();
        if pool == 0 {
            return 0;
        }
        let head = pool.wrapping_add(HEAD_DELTA);
        for &id in &PROBE_IDS[..6] {
            let hit = callee_thiscall!(1, u32, head, id);
            if hit != 0 {
                return hit.wrapping_add(HIT_DELTA);
            }
        }
        let seventh = callee_thiscall!(1, u32, head, PROBE_IDS[6]);
        if seventh != 0 {
            let confirm = callee_thiscall!(1, u32, head, CONFIRM_ID);
            if confirm == 0 {
                return seventh.wrapping_add(EXTENDED_DELTA);
            }
        }
        let last = callee_thiscall!(1, u32, head, LAST_ID);
        if last == 0 {
            0
        } else {
            last.wrapping_add(HIT_DELTA)
        }
    }
});
