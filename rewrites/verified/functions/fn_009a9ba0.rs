// original: 0x009a9ba0 audSpeechManager::alloc_conversation_row
/// Reserves the next free conversation row across three rotating buckets.
///
/// The manager stores three row-map bytes at offsets `0x368..0x36b` and a
/// bucket cursor at `0x36b`. Each attempt selects the cursor modulo three,
/// combines that bucket with its map byte, and checks the row's busy byte.
/// On a free row it marks the row busy, writes the supplied sound identifier
/// into the following word, advances the cursor, and returns the bucket. If
/// all three attempts are busy it returns signed `-1` without changing state.
/// The cursor is an unsigned byte, so the three candidate values remain
/// nonnegative even when it starts at 255.
const ROW_MAP: u32 = 0x368;
const BUCKET_CURSOR: u32 = 0x36B;
const BUCKET_COUNT: u32 = 3;
const ROW_GROUP_STRIDE: u32 = 0x20;
const ROW_BUSY: u32 = 8;
const ROW_SOUND_ID: u32 = 0x0C;
const MAX_ATTEMPTS: u32 = 3;

export!(thiscall, rw_009a9ba0(this_: u32, sound_id: u32) -> i32 {
    let initial_cursor = unsafe {
        (this_.wrapping_add(BUCKET_CURSOR) as *const u8).read() as u32
    };
    let mut candidate = initial_cursor;
    let mut attempts = 0;
    while attempts < MAX_ATTEMPTS {
        let bucket = candidate % BUCKET_COUNT;
        let row_map = unsafe {
            (this_.wrapping_add(ROW_MAP + bucket) as *const u8).read() as u32
        };
        let group = row_map.wrapping_add(BUCKET_COUNT.wrapping_mul(bucket));
        let row_offset = group.wrapping_mul(3).wrapping_mul(ROW_GROUP_STRIDE);
        let busy_address = this_.wrapping_add(row_offset).wrapping_add(ROW_BUSY);
        let is_busy = unsafe { (busy_address as *const u8).read() };
        if is_busy == 0 {
            unsafe {
                (busy_address as *mut u8).write(1);
                (this_.wrapping_add(row_offset).wrapping_add(ROW_SOUND_ID) as *mut u32)
                    .write(sound_id);
                (this_.wrapping_add(BUCKET_CURSOR) as *mut u8)
                    .write(((bucket + 1) % BUCKET_COUNT) as u8);
            }
            return bucket as i32;
        }
        candidate = candidate.wrapping_add(1);
        attempts += 1;
    }
    -1
});
