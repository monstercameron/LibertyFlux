// original: 0x00664890 rage::snMigrateSessionTask::vf11
/// Migrate-session gamer search: claim the first matching idle slot.
///
/// thiscall/2, returns nonzero with the low byte set on a claim, otherwise
/// the count with its low byte cleared. Scans the gamer records for the first
/// one whose id pair matches and whose claimed flag is clear, sets the flag
/// and reports the hit.
export!(thiscall, rw_00664890(this_ptr: u32, lo: u32, hi: u32) -> u32 {
    unsafe {
        let count = ((this_ptr + 0x904) as *const u32).read() as i32;
        if count > 0 {
            let mut i = 0i32;
            while i < count {
                let slot = this_ptr.wrapping_add(0xd8).wrapping_add((i as u32).wrapping_mul(0x40));
                if (slot as *const u32).read() == lo
                    && ((slot + 4) as *const u32).read() == hi
                    && ((this_ptr.wrapping_add(0x8e0).wrapping_add(i as u32)) as *const u8).read() == 0
                {
                    ((this_ptr.wrapping_add(0x8e0).wrapping_add(i as u32)) as *mut u8).write(1);
                    return ((i as u32) & 0xffffff00) | 1;
                }
                i += 1;
            }
            return (count as u32) & 0xffffff00;
        }
        0
    }
});
