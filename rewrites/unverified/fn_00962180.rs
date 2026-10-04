// original: 0x00962180 slot_release_if_flag
/// Release the slot object held by this record.
///
/// When the held object pointer is null, only the record itself is cleared.
/// Otherwise the flag byte at +4 selects the cleared block: a clear flag
/// clears the object's own trailer (+0xA4 = -1, +0xA8/+0xAC = 0), while a
/// set flag clears the trailer of the object its head word points at, and
/// only when that head word is non-null. Always clears the flag byte and
/// the held pointer. Returns the cleared block's pointer, or 0 when nothing
/// was cleared.
export!(thiscall, rw_00962180(this_ptr: u32) -> u32 {
    unsafe {
        let flag = *((this_ptr as *const u8).add(4));
        let obj = *(this_ptr as *const u32);
        let mut ret = 0u32;
        if obj != 0 {
            if flag == 0 {
                *((obj as *mut u32).add(0xAC / 4)) = 0;
                *((obj as *mut u32).add(0xA8 / 4)) = 0;
                *((obj as *mut u32).add(0xA4 / 4)) = 0xFFFFFFFF;
                ret = obj;
            } else {
                let head = *(obj as *const u32);
                if head != 0 {
                    *((head as *mut u32).add(0xAC / 4)) = 0;
                    *((head as *mut u32).add(0xA8 / 4)) = 0;
                    *((head as *mut u32).add(0xA4 / 4)) = 0xFFFFFFFF;
                    ret = head;
                }
            }
        }
        *((this_ptr as *mut u8).add(4)) = 0;
        *(this_ptr as *mut u32) = 0;
        ret
    }
});
