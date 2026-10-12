// original: 0x0087A050 rage::crmtComposerOptimized::remove_track

/// Remove the track at the list's current count boundary from a composer.
///
/// `this` points to a composer whose list pointer is at `+0x0c`; the list
/// stores its track array at `+0x04` and its unsigned count at `+0x14`.
/// The function increments the composer's 16-bit operation counter at
/// `+0x72`, obtains the frame at the list's current index, and dispatches the
/// track stored at `tracks[count]` together with the operation argument.
/// After dispatch it swaps `tracks[count - 1]` with `tracks[count]`, decrements
/// the list count with 32-bit wrapping, and returns the list pointer. The
/// array access at `count - 1` wraps to the preceding word when count is zero.
///
/// Original: thiscall with one stack word; the helper calls are intercepted.
lf_checker_rt::export!(thiscall, rw_0087a050(this: u32, operation: u32) -> u32 {
    const LIST_OFFSET: u32 = 0x0c;
    const TRACK_ARRAY_OFFSET: u32 = 0x04;
    const TRACK_COUNT_OFFSET: u32 = 0x14;
    const OPERATION_COUNT_OFFSET: u32 = 0x72;
    const GET_FRAME: u32 = 1;
    const DISPATCH_FRAME: u32 = 2;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn increment_u16(address: u32) {
        unsafe {
            let value = (address as *const u16).read_unaligned();
            (address as *mut u16).write_unaligned(value.wrapping_add(1));
        }
    }

    unsafe {
        let list = read_u32(this.wrapping_add(LIST_OFFSET));
        increment_u16(this.wrapping_add(OPERATION_COUNT_OFFSET));

        let count = read_u32(list.wrapping_add(TRACK_COUNT_OFFSET));
        let tracks = read_u32(list.wrapping_add(TRACK_ARRAY_OFFSET));
        let current_track = read_u32(tracks.wrapping_add(count.wrapping_mul(4)));

        let frame_list = lf_checker_rt::callee_thiscall!(
            GET_FRAME,
            u32,
            list.wrapping_add(TRACK_ARRAY_OFFSET),
            list
        );
        let mut frame_slot = frame_list;
        let frame_slot_address = (&mut frame_slot as *mut u32) as u32;
        let _ = lf_checker_rt::callee_thiscall!(
            DISPATCH_FRAME,
            u32,
            frame_slot_address,
            current_track,
            operation
        );

        let previous_index = count.wrapping_sub(1);
        let previous_track = read_u32(tracks.wrapping_add(previous_index.wrapping_mul(4)));
        write_u32(tracks.wrapping_add(previous_index.wrapping_mul(4)), current_track);
        write_u32(tracks.wrapping_add(count.wrapping_mul(4)), previous_track);
        write_u32(list.wrapping_add(TRACK_COUNT_OFFSET), previous_index);
        list
    }
});
