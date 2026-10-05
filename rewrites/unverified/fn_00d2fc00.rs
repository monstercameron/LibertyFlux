// original: 0x00d2fc00 task_route_box_collect (proposed)

/// Scan route entries inside a 0.1 box of a moving center, reporting hits.
///
/// `this` points to the task object: dword at `+0x70` is the route table
/// (i32 count at `[0]`, entries of 16 bytes from `+0x10`, the point at
/// `+0/+4/+8` of each entry) and u16 tags from `+0x78` (one per entry, four
/// bytes apart). `arg` points to an object whose dword at `+0x224` is a
/// second object holding the starting center (floats at `+0x290/0x294/0x298`)
/// and a fourth word at `+0x29c` that travels with it.
///
/// Entries whose tag has none of the low three bits set are skipped. Any
/// other entry whose point lies within `HALF_RANGE` (0.1) of the center on
/// all three lanes (NaN on any compared lane skips the entry) is reported to
/// the collector callee (thiscall on the second object, one stack word: a
/// pointer to the four-word center buffer), which may move the center; the
/// scan continues with the updated center. The count is re-read every
/// iteration; a non-positive count runs zero iterations. Returns nothing.
///
/// Original: 0x00d2fc00 (thiscall: object in ECX, one stack word, callee pops
/// 4, no return value).
lf_checker_rt::export!(thiscall, rw_00d2fc00(this: u32, arg: u32) -> u32 {
    unsafe {
        const TABLE_PTR: u32 = 0x70;
        const TAGS: u32 = 0x78;
        const TAG_STRIDE: i32 = 4;
        const OBJ_LINK: u32 = 0x224;
        const CENTER_X: u32 = 0x290;
        const CENTER_Y: u32 = 0x294;
        const CENTER_Z: u32 = 0x298;
        const CENTER_W: u32 = 0x29c;
        const ENTRY_BASE: i32 = 0x10;
        const ENTRY_STRIDE: i32 = 0x10;
        const HALF_RANGE: f32 = f32::from_bits(0x3dcccccd); // 0.1
        const COLLECTOR: u32 = 0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let obj2 = rd32(arg + OBJ_LINK);
        let mut center = [
            rd32(obj2 + CENTER_X),
            rd32(obj2 + CENTER_Y),
            rd32(obj2 + CENTER_Z),
            rd32(obj2 + CENTER_W),
        ];
        let table = rd32(this + TABLE_PTR);
        if (rd32(table) as i32) <= 0 {
            return 0;
        }
        let mut i = 0i32;
        loop {
            let tag = ((this.wrapping_add(TAGS).wrapping_add((i.wrapping_mul(TAG_STRIDE)) as u32))
                as *const u16)
                .read_unaligned();
            if tag as u8 & 7 != 0 {
                let ex = table.wrapping_add((i.wrapping_mul(ENTRY_STRIDE).wrapping_add(ENTRY_BASE)) as u32);
                let px = f32::from_bits(center[0]);
                let py = f32::from_bits(center[1]);
                let pz = f32::from_bits(center[2]);
                let x = rdf(ex);
                let y = rdf(ex + 4);
                let z = rdf(ex + 8);
                // In range on a lane iff center >= v - HALF and v + HALF >=
                // center, both ordered (a NaN anywhere fails the lane).
                let in_x = px >= sub(x, HALF_RANGE) && add(x, HALF_RANGE) >= px;
                let in_y = py >= sub(y, HALF_RANGE) && add(y, HALF_RANGE) >= py;
                let in_z = pz >= sub(z, HALF_RANGE) && add(z, HALF_RANGE) >= pz;
                if in_x && in_y && in_z {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        COLLECTOR,
                        u32,
                        obj2,
                        center.as_mut_ptr() as u32
                    );
                }
            }
            i = i.wrapping_add(1);
            let live_table = rd32(this + TABLE_PTR);
            if i >= (rd32(live_table) as i32) {
                break;
            }
        }
        0
    }
});
