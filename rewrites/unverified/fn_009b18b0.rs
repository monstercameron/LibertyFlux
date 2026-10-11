// original: 0x009B18B0 audPoliceScanner::maybe_play_dispatch_update

/// Scans 32 dispatch records for active entries that are younger than 2,000
/// milliseconds and have a signed crime type in 0..16. It resolves the type's
/// three-word sound position, clears the record's active byte, then skips the
/// dispatch when a zero marker and subtype at most two are accompanied by a
/// signed wanted-level result of at least two. Otherwise it forwards the type,
/// subtype, marker and position to the scanner helper. The three time and type
/// tests use unsigned wrapping time arithmetic and signed type bounds.
lf_checker_rt::export!(thiscall, rw_009b18b0(this: u32) -> u32 {
    const DISPATCH_RECORDS: u32 = 0x3470;
    const RECORD_COUNT: u32 = 32;
    const RECORD_STRIDE: u32 = 0x14;
    const CRIME_TYPE: u32 = 0x00;
    const SUBTYPE: u32 = 0x04;
    const MARKER: u32 = 0x08;
    const EVENT_TIME: u32 = 0x0c;
    const ACTIVE: u32 = 0x10;
    const MAX_AGE_MS: u32 = 0x7d0;
    const CURRENT_TIME: u32 = 0x011735b4;
    const TYPE_LIMIT: i32 = 16;
    const WANTED_THRESHOLD: i32 = 2;

    unsafe {
        let current_time = lf_checker_rt::global::<u32>(CURRENT_TIME).read_unaligned();

        for index in 0..RECORD_COUNT {
            let record = this
                .wrapping_add(DISPATCH_RECORDS)
                .wrapping_add(index.wrapping_mul(RECORD_STRIDE));
            if (record.wrapping_add(ACTIVE) as *const u8).read() == 0 {
                continue;
            }

            let event_time = (record.wrapping_add(EVENT_TIME) as *const u32).read_unaligned();
            if event_time.wrapping_add(MAX_AGE_MS) >= current_time {
                continue;
            }

            let crime_type = (record.wrapping_add(CRIME_TYPE) as *const u32).read_unaligned();
            let signed_crime_type = crime_type as i32;
            if signed_crime_type < 0 || signed_crime_type >= TYPE_LIMIT {
                continue;
            }

            // The callee returns a pointer to the three float words copied to
            // the temporary stack record. Keep their bits intact.
            let sound_row = lf_checker_rt::callee_cdecl!(1, u32, 1, crime_type);
            let mut sound_position = [
                (sound_row as *const u32).read_unaligned(),
                (sound_row.wrapping_add(4) as *const u32).read_unaligned(),
                (sound_row.wrapping_add(8) as *const u32).read_unaligned(),
            ];

            let subtype = (record.wrapping_add(SUBTYPE) as *const u32).read_unaligned();
            let marker = (record.wrapping_add(MARKER) as *const u32).read_unaligned();
            (record.wrapping_add(ACTIVE) as *mut u8).write(0);

            if marker == 0 && subtype <= 2 {
                let player_context = lf_checker_rt::callee_cdecl!(2, u32, 0);
                let player_info = (player_context.wrapping_add(0x228) as *const u32)
                    .read_unaligned();
                let wanted_object = if player_info == 0 {
                    0
                } else {
                    player_info.wrapping_add(0x70)
                };
                let wanted_result = lf_checker_rt::callee_thiscall!(3, u32, wanted_object);
                if (wanted_result as i32) >= WANTED_THRESHOLD {
                    continue;
                }
            }

            let position_pointer = sound_position.as_mut_ptr() as usize as u32;
            let _ = lf_checker_rt::callee_thiscall!(
                4,
                u32,
                this,
                subtype,
                marker,
                position_pointer
            );
        }

        // The original leaves EAX holding loop scratch state.
        0
    }
});
