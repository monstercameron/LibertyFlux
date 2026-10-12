// original: 0x009a6390 script_audio_slots_idle
/// Reports whether the two active script-audio slots are idle.
///
/// The routine reads a 32-bit manager pointer from a relocated global. A null
/// manager means idle. Otherwise it checks two pointers at manager offset
/// `0x20`; null entries are ignored. A non-null entity blocks the idle result
/// only when its byte at `0x47` is zero and its unsigned word at `0x08` is
/// also zero. The result is returned in AL.
const MANAGER_POINTER_VA: u32 = 0x0128_8520;
const SLOT_ARRAY_OFFSET: u32 = 0x20;
const SLOT_COUNT: u32 = 2;
const ENTITY_ACTIVE_FLAG: u32 = 0x47;
const ENTITY_PENDING_COUNT: u32 = 0x08;

export!(cdecl, rw_009a6390() -> u8 {
    let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_POINTER_VA).read() };
    let mut idle = true;
    if manager != 0 {
        let mut slot = manager.wrapping_add(SLOT_ARRAY_OFFSET);
        let mut remaining = SLOT_COUNT;
        while remaining != 0 {
            let entity = unsafe { (slot as *const u32).read() };
            if entity != 0 {
                let active = unsafe { (entity.wrapping_add(ENTITY_ACTIVE_FLAG) as *const u8).read() };
                if active == 0 {
                    let pending = unsafe {
                        (entity.wrapping_add(ENTITY_PENDING_COUNT) as *const u32).read()
                    };
                    if pending == 0 {
                        idle = false;
                    }
                }
            }
            slot = slot.wrapping_add(4);
            remaining -= 1;
        }
    }
    u8::from(idle)
});
