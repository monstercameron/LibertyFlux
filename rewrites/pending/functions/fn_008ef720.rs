// original: 0x008ef720 init_input_state_element
use lf_checker_rt::{callee_thiscall, export, relocated};

/// Initialise one input-state element (original 0x008EF720).
///
/// Zeroes five embedded sub-objects through the shared two-word clearer,
/// stamps 187 sixteen-byte binding records plus five trailing records with
/// the default-table pointer, arms one flag word, then publishes the tail
/// markers. Returns the element pointer. The guarded wipe behind the last
/// record's pointer only runs when that pointer is non-null, which the
/// initialisation above never leaves it.
export!(thiscall, rw_008ef720(this: u32) -> u32 {
    const SUB_OBJECTS: [u32; 5] = [0x0, 0x7B8, 0xF70, 0x1728, 0x1EE0];
    // Do-while with the decrement first: 0xBA reaches -1 after 187 bodies.
    const RECORDS: usize = 187;
    const RECORD_BASE: u32 = 0x2698;
    const RECORD_LEN: u32 = 0x10;
    const TRAIL_BASE: u32 = 0x3248;
    const LAST_RECORD: u32 = 0x3290;
    const LAST_SUB: u32 = 0x32AC;
    const DEFAULT_TABLE: u32 = 0x00E837F0;
    for off in SUB_OBJECTS {
        callee_thiscall!(1, u32, this.wrapping_add(off));
    }
    let table = relocated(DEFAULT_TABLE);
    unsafe {
        let base = this as *mut u8;
        let mut rec = base.add(RECORD_BASE as usize) as *mut u32;
        for _ in 0..RECORDS {
            write_record(rec, table);
            rec = rec.add((RECORD_LEN / 4) as usize);
        }
        let mut trail = base.add(TRAIL_BASE as usize) as *mut u32;
        for _ in 0..4 {
            write_record(trail, table);
            trail = trail.add((RECORD_LEN / 4) as usize);
        }
        write_record(base.add(LAST_RECORD as usize) as *mut u32, table);
        callee_thiscall!(1, u32, this.wrapping_add(LAST_SUB));
        (base.add(0x328C) as *mut u16).write(1);
        base.add(0x328E).write(0);
        (base.add(0x3296) as *mut u16).write(0);
        let wipe_at = (base.add(0x329C) as *const u32).read();
        if wipe_at != 0 {
            // Guarded wipe: dead after the initialisation above, kept exact.
            let mut done: u32 = 0;
            while done != 0x200 {
                done = done.wrapping_add(8);
                ((wipe_at.wrapping_add(done).wrapping_sub(8)) as *mut u8).write(0);
                ((wipe_at.wrapping_add(done).wrapping_sub(4)) as *mut u32).write(0);
            }
        }
        (base.add(0x32A0) as *mut u32).write(0);
        (base.add(0x3A80) as *mut u32).write(0);
        (base.add(0x32A8) as *mut u32).write(0xFFFF_FFFF);
    }
    this
});

/// One sixteen-byte binding record: table pointer, zero word, zero byte,
/// zero word. Bytes 9..12 of the record are left untouched.
#[inline(always)]
fn write_record(rec: *mut u32, table: u32) {
    unsafe {
        rec.write(table);
        rec.add(1).write(0);
        (rec as *mut u8).add(8).write(0);
        rec.add(3).write(0);
    }
}
