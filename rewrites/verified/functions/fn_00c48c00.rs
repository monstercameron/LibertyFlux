// original: 0x00c48c00 ccamcinematic_init_tables (proposed)
/// Initialise the cinematic tables to idle, then load the shared set.
///
/// Resets the shared scale to 1.0, zeroes the counters at `this + 0x1f0`,
/// `0x200`, `0x204` (word) and `0x206`/`0x207` (bytes), stamps the tick
/// global at `this + 0x1f4`, and fills both 21-word tables at
/// `this + TABLE_A`/`TABLE_B` with `EMPTY` (-1). Writes the scalar
/// defaults `FOV` at +0x1e8, `0x41cb3333` at +0x1ec, `20000` at +0x1f8
/// and the shared slot count at +0x1fc. When the shared active flag is
/// set, derives the slot count from the shared base, takes the count
/// and mode byte from the shared block, and copies the 21-word shared
/// tables over both local tables. The shared tables form one 42-word
/// run at `SHARED_TABLES`: the first 21 words feed table A, the next
/// 21 feed table B (the original addresses B through `edi`, which
/// works out to `SHARED_TABLES + 0x54`). Returns 1 in the low byte.
///
/// Original: thiscall, no stack arguments. (True size 220 bytes: the
/// inventory's 218 stops at the `(an instruction of the original)`, missing `(an instruction of the original); ret`.)
lf_checker_rt::export!(thiscall, rw_00c48c00(this: u32) -> u32 {
    const SCALE_GLOBAL: u32 = 0x01048f4c;
    const TICK_GLOBAL: u32 = 0x011735c4;
    const SLOT_COUNT_GLOBAL: u32 = 0x01048f50;
    const ACTIVE_FLAG: u32 = 0x016d8b40;
    const SHARED_BASE: u32 = 0x016d8d4c;
    const SHARED_COUNT: u32 = 0x016d8d50;
    const SHARED_MODE: u32 = 0x016d8d56;
    const SHARED_TABLES: u32 = 0x016d8c90;
    const SHARED_TABLE_B_OFF: u32 = 0x54;
    const ONE: u32 = 0x3f80_0000;
    const EMPTY: u32 = 0xffff_ffff;
    const TABLE_A: u32 = 0x140;
    const TABLE_B: u32 = 0x194;
    const TABLE_WORDS: u32 = 0x15;
    const FOV_DEFAULT: u32 = 0x42e4_9999;
    const SPAN_DEFAULT: u32 = 0x41cb_3333;
    const FAR_DEFAULT: u32 = 20000;
    unsafe {
        lf_checker_rt::global::<u32>(SCALE_GLOBAL).write_unaligned(ONE);
        ((this + 0x207) as *mut u8).write(0);
        ((this + 0x1f0) as *mut u32).write_unaligned(0);
        let tick = lf_checker_rt::global::<u32>(TICK_GLOBAL).read_unaligned();
        ((this + 0x1f4) as *mut u32).write_unaligned(tick);
        ((this + 0x1f8) as *mut u32).write_unaligned(FAR_DEFAULT);
        let mut i = 0u32;
        while i < TABLE_WORDS {
            ((this + TABLE_A + i * 4) as *mut u32).write_unaligned(EMPTY);
            ((this + TABLE_B + i * 4) as *mut u32).write_unaligned(EMPTY);
            i += 1;
        }
        ((this + 0x200) as *mut u32).write_unaligned(0);
        let slots = lf_checker_rt::global::<u32>(SLOT_COUNT_GLOBAL).read_unaligned();
        ((this + 0x1fc) as *mut u32).write_unaligned(slots);
        ((this + 0x1e8) as *mut u32).write_unaligned(FOV_DEFAULT);
        ((this + 0x204) as *mut u16).write_unaligned(0);
        ((this + 0x1ec) as *mut u32).write_unaligned(SPAN_DEFAULT);
        ((this + 0x206) as *mut u8).write(0);
        if lf_checker_rt::global::<u8>(ACTIVE_FLAG).read() != 0 {
            let base = lf_checker_rt::global::<u32>(SHARED_BASE).read_unaligned();
            lf_checker_rt::global::<u32>(SLOT_COUNT_GLOBAL).write_unaligned(base.wrapping_sub(1));
            let n = lf_checker_rt::global::<u32>(SHARED_COUNT).read_unaligned();
            ((this + 0x200) as *mut u32).write_unaligned(n);
            let mode = lf_checker_rt::global::<u8>(SHARED_MODE).read();
            ((this + 0x206) as *mut u8).write(mode);
            let ta = lf_checker_rt::relocated(SHARED_TABLES);
            let tb = lf_checker_rt::relocated(SHARED_TABLES + SHARED_TABLE_B_OFF);
            i = 0;
            while i < TABLE_WORDS {
                let a = ((ta + i * 4) as *const u32).read_unaligned();
                ((this + TABLE_A + i * 4) as *mut u32).write_unaligned(a);
                let b = ((tb + i * 4) as *const u32).read_unaligned();
                ((this + TABLE_B + i * 4) as *mut u32).write_unaligned(b);
                i += 1;
            }
        }
    }
    1
});
