// original: 0x0088fb20 audio_update_play_cursor
/// Advances this voice's playback cursor from the entity table.
///
/// Resolves the voice selector at byte `this+4` through the global entity
/// table; a table value of -1 (or an empty selector) keeps the current
/// cursor at `this+0x88`. A cursor of -1 restarts playback through the
/// helper; otherwise the cursor advances by the frame count argument and
/// the previous cursor is kept alongside it.
export!(thiscall, rw_0088fb20(this_ptr: *mut u8, frames: u32) -> u32 {
    unsafe {
        *this_ptr.add(0x38) |= 0x60;
        let sel = *this_ptr.add(4);
        if sel != 0xFF {
            let stride = *(relocated(0x0115D968) as *const u32);
            let base = *(relocated(0x0115D988) as *const u32) as *const u8;
            let row_sel = *this_ptr.add(0x40) as u32;
            let row = base.add(row_sel.wrapping_mul(0x6F40) as usize);
            let entry = stride
                .wrapping_mul(sel as u32)
                .wrapping_add(*(row.add(0x6F14) as *const u32));
            let v = *((entry as *const u8).add(0xE0) as *const u32);
            if v != 0xFFFFFFFF {
                *(this_ptr.add(0x88) as *mut u32) = v;
            }
        }
        let cursor = *(this_ptr.add(0x88) as *const u32);
        if cursor == 0xFFFFFFFF {
            *(this_ptr.add(0x88) as *mut u32) = 0;
            let ans: u32 = callee_thiscall!(1, u32, this_ptr as u32);
            *this_ptr.add(0x39) |= 8;
            ans
        } else {
            *(this_ptr.add(0x8C) as *mut u32) = cursor.wrapping_add(frames);
            *(this_ptr.add(0x88) as *mut u32) = frames;
            cursor.wrapping_add(frames)
        }
    }
});
