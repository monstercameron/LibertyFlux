// original: 0x009268B0 ui_slot_constants_upload (proposed)
use lf_checker_rt::{callee_cdecl, export, global, relocated};

/// Upload the constant vectors for one slot of the input-UI effect tables.
///
/// `slot` selects the entry: values below 8 index the 16-byte direct table
/// and the 0x110-byte records of the near table, values of 8 and up index
/// the 0x100-byte records of the far table (the buffer parity word picks
/// which half of either table is current). The function first uploads the
/// scene-wide vector (three globals and a zero) through the first handle,
/// then derives four more constant vectors from the selected record and
/// uploads each through the shared register routine, then uploads a pointer
/// into the record itself and finally the accumulated four-float block.
///
/// Returns nothing. `slot` is signed; negative values take the near-table
/// path with a negative offset and are not meaningful.
export!(cdecl, rw_9268b0(slot: i32) -> () {
    unsafe {
        const SCENE_VEC: u32 = 0x0119D004; // three floats
        const SCENE_HANDLE: u32 = 0x0119D054;
        const OVERRIDE_FLAG: u32 = 0x0119D010; // byte
        const OVERRIDE_A: u32 = 0x0119D014;
        const OVERRIDE_B: u32 = 0x0119D018;
        const OVERRIDE_C: u32 = 0x0119D024;
        const SCALE: u32 = 0x0119D02C;
        const BUFFER_PARITY: u32 = 0x01174794;
        const DIRECT_TABLE: u32 = 0x0119F980; // 16-byte entries (4 floats)
        const RECORDS_NEAR: u32 = 0x0119FA00; // 0x110-byte records
        const RECORDS_FAR: u32 = 0x0119D0F0; // 0x100-byte records
        const SHARED_HANDLE: u32 = 0x0154E190;
        const REG_POSITION: u32 = 0x0154E250;
        const REG_PARAMS: u32 = 0x0154E254;
        const REG_RADII: u32 = 0x0154E258;
        const REG_BLOCK: u32 = 0x0154E25C;
        const REG_RAW: u32 = 0x0154E260;
        const FIRST_FAR_SLOT: i32 = 8;
        const NEAR_STRIDE: i32 = 0x110;

        let f32_at = |va: u32| global::<f32>(va).read();
        let u32_at = |va: u32| global::<u32>(va).read();
        let ptr = |v: &[f32; 4]| v.as_ptr() as u32;

        // Scene-wide vector.
        let scene = [f32_at(SCENE_VEC), f32_at(SCENE_VEC + 4), f32_at(SCENE_VEC + 8), 0.0f32];
        callee_cdecl!(1, u32, u32_at(SCENE_HANDLE), ptr(&scene));

        let parity = global::<i32>(BUFFER_PARITY).read();
        let (rec, mut block): (u32, [f32; 4]) = if slot < FIRST_FAR_SLOT {
            let rec = slot
                .wrapping_add(parity.wrapping_mul(8))
                .wrapping_mul(NEAR_STRIDE)
                .wrapping_add(RECORDS_NEAR as i32) as u32;
            let d = DIRECT_TABLE.wrapping_add((slot as u32) << 4);
            (rec, [f32_at(d), f32_at(d + 4), f32_at(d + 8), f32_at(d + 12)])
        } else {
            let index = slot.wrapping_sub(FIRST_FAR_SLOT).wrapping_add(parity.wrapping_shl(4));
            let rec = ((index as u32) << 8).wrapping_add(RECORDS_FAR);
            (rec, [f32_at(rec + 0xC0), f32_at(rec + 0xC4), f32_at(rec + 0xC8), f32_at(rec + 0xCC)])
        };
        let rf = |off: u32| f32_at(rec.wrapping_add(off));

        // Record position and inverse extent.
        let position = [rf(0x2C), rf(0x30), rf(0x34), -1.0f32 / rf(0x14)];
        callee_cdecl!(2, u32, u32_at(SHARED_HANDLE), u32_at(REG_POSITION), ptr(&position));

        // Cone parameters come from the global override or from the record.
        let (cone_a, cone_b, base) = if global::<u8>(OVERRIDE_FLAG).read() != 0 {
            (f32_at(OVERRIDE_C), f32_at(OVERRIDE_B), f32_at(OVERRIDE_A))
        } else {
            (rf(0x20), rf(0x1C), rf(0x24))
        };
        let count = global::<i32>(rec.wrapping_add(0x04)).read();
        let inverse = 1.0f32 / (count as f32);
        let weight = (f32_at(SCALE) * inverse) * block[3];
        let params = [cone_a, cone_b, inverse, base];
        block[2] = base * inverse;
        block[0] = weight + block[0];
        block[1] = weight + block[1];
        callee_cdecl!(3, u32, u32_at(SHARED_HANDLE), u32_at(REG_PARAMS), ptr(&params));

        // Radii and counts of the record.
        let width = global::<i32>(rec.wrapping_add(0x08)).read();
        let radii = [rf(0x28), width as f32, rf(0x10), rf(0x14)];
        callee_cdecl!(4, u32, u32_at(SHARED_HANDLE), u32_at(REG_RADII), ptr(&radii));

        // Raw words inside the record.
        callee_cdecl!(5, u32, u32_at(SHARED_HANDLE), u32_at(REG_RAW), relocated(rec.wrapping_add(0x80)));

        // Accumulated block.
        callee_cdecl!(6, u32, u32_at(SHARED_HANDLE), u32_at(REG_BLOCK), ptr(&block));
    }
});
