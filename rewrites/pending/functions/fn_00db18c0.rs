// original: 0x00db18c0 UILayoutFrame::vf63
/// Release every live entry in the frame's pointer list, then report it.
///
/// Walks the (base, count) list stored on the object. Each non-null entry
/// is released through the entry's own teardown and then freed; null
/// entries are skipped. The list bounds are re-read every iteration, and
/// once the walk finishes the final base and end are reported to the
/// list's owner callback.
export!(thiscall, rw_00db18c0(this_ptr: u32) -> u32 {
    unsafe {
        const LIST_BASE: usize = 0xbc;
        const LIST_COUNT: usize = 0xc0;

        let obj = this_ptr as *const u8;
        let base_of = |o: *const u8| -> u32 { *(o.add(LIST_BASE) as *const u32) };
        let count_of = |o: *const u8| -> u32 {
            *(o.add(LIST_COUNT) as *const u16) as u32
        };
        let end_of = |o: *const u8| -> u32 {
            base_of(o).wrapping_add(count_of(o).wrapping_mul(4))
        };

        let mut cursor = base_of(obj);
        if cursor != end_of(obj) {
            loop {
                let entry = *(cursor as *const u32);
                if entry != 0 {
                    callee_thiscall!(1, u32, entry);
                    callee_cdecl!(2, u32, entry);
                }
                cursor = cursor.wrapping_add(4);
                if cursor == end_of(obj) {
                    break;
                }
            }
        }
        callee_thiscall!(
            3,
            u32,
            (this_ptr as usize).wrapping_add(LIST_BASE) as u32,
            base_of(obj),
            end_of(obj)
        );
        0
    }
});
