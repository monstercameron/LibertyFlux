// original: 0x00925EB0 ui_effect_tables_init (proposed)
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Reset the input-UI effect tables to their start-up state.
///
/// Takes no arguments and returns nothing. It fills three global tables and
/// a block of scalar settings:
///
/// * The near table: eight 0x110-byte records. Each record is cleared and
///   its record index is stored as an atlas cell (column = index mod 4 and
///   row = 2 + index / 4, each times a quarter), then the shared record
///   initialiser is called on the embedded sub-object with a fixed 14-word
///   argument list.
/// * The far table: sixteen 0x100-byte records, each initialised by one call
///   that takes a pointer into the near table (alternating between its first
///   two records) and given a cell of (index mod 4, index / 4) quarters.
/// * The scene scalars: tint, cone, ramp and flag globals set to their
///   defaults, the first four-float vector set to ones, and the renderer
///   pointers cleared.
///
/// Finally two small descriptors are built by one routine each (each with a
/// callback pointer) and handed to a registration routine together with the
/// code address `CALLBACK_THUNK` as the fourth word of each.
///
/// One quirk is kept on purpose: the original stores two floats into every
/// near record that it never initialised itself (stack residue). Those two
/// values are not defined by the program, so this rewrite writes zero.
export!(cdecl, rw_925eb0() -> () {
    unsafe {
        const NEAR_TABLE: u32 = 0x0119F1C8; // first record's anchor field
        const NEAR_STRIDE: u32 = 0x110;
        const NEAR_COUNT: u32 = 8;
        const CELLS: u32 = 0x0119F988; // 16-byte cell entries, one per near record
        const FAR_TABLE: u32 = 0x011A0BD8;
        const FAR_STRIDE: u32 = 0x100;
        const FAR_COUNT: u32 = 16;
        const PARITY_SOURCES: u32 = 0x0119F320; // two source records for the far table
        const UNIT_VECTOR: u32 = 0x0119F0F0; // four floats
        const SCENE: u32 = 0x0119D000;
        const QUARTER: f32 = 0.25;
        const UNDEFINED_STACK_FLOAT: u32 = 0; // residue in the original
        const NONE: u32 = 0xFFFF_FFFF;
        const CALLBACK_A: u32 = 0x009256E0;
        const CALLBACK_B: u32 = 0x009256F0;
        const CALLBACK_THUNK: u32 = 0x00430260;

        let put = |va: u32, v: u32| global::<u32>(va).write(v);
        let put8 = |va: u32, v: u8| global::<u8>(va).write(v);
        let quarter = |n: u32| ((n as i32) as f32 * QUARTER).to_bits();

        // Near table.
        for i in 0..NEAR_COUNT {
            let rec = NEAR_TABLE + i * NEAR_STRIDE;
            let cell = CELLS + i * 0x10;
            put(rec + 4, UNDEFINED_STACK_FLOAT);
            put(rec - 8, 0);
            put(rec - 4, 0);
            put(rec, 0);
            put(rec + 8, 0);
            put(rec + 0xC, 0);
            put(rec + 0x10, 0);
            put(rec + 0x14, UNDEFINED_STACK_FLOAT);
            put8(rec + 0x25, 0);
            put(rec + 0x28, NONE);
            put(rec + 0x2C, NONE);
            put(rec + 0x30, 0);
            put(rec + 0x34, NONE);
            put(rec + 0x38, 0);
            callee_thiscall!(1, u32, relocated(rec - 0xC8),
                0, 0, 1.0f32.to_bits(), 256.0f32.to_bits(), 16.0f32.to_bits(),
                0, 0, 0, 0.5f32.to_bits(), 0, 0, (-0.025f32).to_bits(), 0, 5);
            // Cell: column and row of the record in a four-wide atlas.
            let index = 8 + i;
            put(cell - 8, quarter(index % 4));
            put(cell, 0);
            put(cell - 4, quarter(index / 4));
            put(cell + 4, QUARTER.to_bits());
        }

        // Scene scalars.
        put(0x0119D004, 0x3F20_C49C);
        put(0x0119D008, 1.0f32.to_bits());
        put(0x0119D00C, 1.0f32.to_bits());
        put8(0x0119D013, 0);
        put8(0x0119D090, 0);
        put8(0x0119D091, 0);
        put8(0x0119D092, 0);
        put8(0x0119D093, 1);
        put8(0x0119D095, 0);
        put8(0x0119D010, 1);
        put(0x0119D014, 3.0f32.to_bits());
        put(0x0119D018, 0.1f32.to_bits());
        put(0x0119D020, 0.3f32.to_bits());
        put(0x0119D024, (-0.1f32).to_bits());
        put(0x0119D028, core::f32::consts::FRAC_PI_2.to_bits());
        put(0x0119D02C, 0.1f32.to_bits());
        for k in 0..4 {
            put(UNIT_VECTOR + 4 * k, 1.0f32.to_bits());
        }
        put(0x0119D01C, 5);
        put8(0x0119D011, 0);
        put(0x0119D030, 0);
        put8(0x0119D012, 0);
        put(0x0119CFF8, 0);
        put(0x0119CFFC, 0);
        put(SCENE, 0);
        put(0x0119CFE4, 7);

        // Far table.
        for i in 0..FAR_COUNT {
            let rec = FAR_TABLE + i * FAR_STRIDE;
            let source = PARITY_SOURCES + (i & 1) * 0x110;
            callee_thiscall!(2, u32, relocated(rec - 0xC8), relocated(source));
            put(rec + 0x14, 0);
            put(rec + 0x18, 0);
            put(rec + 0x1C, NONE);
            put(rec + 0x20, NONE);
            put(rec + 0x24, NONE);
            put(rec + 0x28, 0);
            put(rec, 0);
            put(rec + 4, QUARTER.to_bits());
            put(rec - 8, quarter(i % 4));
            put(rec - 4, quarter(i / 4));
        }

        // Two callback descriptors, then their registration.
        put(0x0119CFF0, 0);
        put(0x0119CFF4, 0);
        put(0x0119D098, 0);
        put(0x0119D09C, NONE);
        put(0x0119D0A0, NONE);
        put(0x0119D0A4, 0);
        let mut first = [0u32; 4];
        callee_thiscall!(3, u32, first.as_mut_ptr() as u32, 0, relocated(CALLBACK_A), 0, 0);
        first[3] = relocated(CALLBACK_THUNK);
        let mut second = [0u32; 4];
        callee_thiscall!(4, u32, second.as_mut_ptr() as u32, 0, relocated(CALLBACK_B), 0, 0);
        second[3] = relocated(CALLBACK_THUNK);
        callee_cdecl!(5, u32,
            first[0], first[1], first[2], first[3],
            second[0], second[1], second[2], second[3]);
    }
});
