// original: 0x00B44FB0 CEventBuildingCollision::CEventBuildingCollision
/// Construct a building-collision event. Two three-word records are copied
/// into the object, along with a 16-bit event kind and the collision-time bit
/// pattern. The base event constructor runs before these fields are written.
lf_checker_rt::export!(thiscall, rw_00b44fb0(
    this: u32, event_kind: u32, collision_time_bits: u32,
    first_record: u32, second_record: u32, final_kind: u32
) -> u32 {
    unsafe {
        const EVENT_VTABLE: u32 = 0x00eadb4c;
        const EVENT_KIND: usize = 0x0c;
        const COLLISION_TIME: usize = 0x10;
        const FIRST_RECORD: usize = 0x20;
        const SECOND_RECORD: usize = 0x30;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        let object = this as *mut u8;
        (object.add(EVENT_KIND) as *mut u16).write_unaligned(event_kind as u16);
        (object.add(EVENT_KIND + 2) as *mut u16).write_unaligned(final_kind as u16);
        (object.add(COLLISION_TIME) as *mut u32).write_unaligned(collision_time_bits);
        for (source, destination_offset) in [
            (first_record, FIRST_RECORD),
            (second_record, SECOND_RECORD),
        ] {
            for index in 0..3u32 {
                let word = ((source as *const u32).add(index as usize)).read_unaligned();
                (object.add(destination_offset) as *mut u32)
                    .add(index as usize).write_unaligned(word);
            }
        }
        (object as *mut u32).write_unaligned(lf_checker_rt::relocated(EVENT_VTABLE));
        this
    }
});
