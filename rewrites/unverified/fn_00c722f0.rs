// original: 0x00c722f0 append_animation_tracks

/// Dispatch the selected track range to the track-copy helper, then add the
/// signed pointer distance from destination to source in 0x30-byte tracks to
/// the object's 16-bit count at `+4`. The range end is the object's track
/// buffer pointer plus its prior count times 0x30. The helper call is part of
/// the observable behavior and is kept as a stdcall call with three words.
lf_checker_rt::export!(thiscall, rw_00c722f0(this: u32, destination: u32, source_start: u32) -> u32 {
    const TRACK_STRIDE: u32 = 0x30;
    const COPY_TRACKS: u32 = 1;
    unsafe {
        let item_count = ((this.wrapping_add(4)) as *const u16).read_unaligned();
        let track_buffer = (this as *const u32).read_unaligned();
        let source_end = track_buffer.wrapping_add(u32::from(item_count).wrapping_mul(TRACK_STRIDE));
        let _ = lf_checker_rt::callee_stdcall!(COPY_TRACKS, u32, source_start, source_end, destination);
        let tracks_added = (source_start.wrapping_sub(destination) as i32) / TRACK_STRIDE as i32;
        let updated_count = item_count.wrapping_add(tracks_added as u16);
        ((this.wrapping_add(4)) as *mut u16).write_unaligned(updated_count);
    }
    destination
});
