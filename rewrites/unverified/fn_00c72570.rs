// original: 0x00c72570 ped_task_spatial_box_find (proposed)

/// Find the first spatial query result whose box contains a point.
///
/// `point` points to three floats (x, y, z). The routine reads the spatial
/// index from a game global and returns 0 when it is null; otherwise it asks
/// the index for the results near `point` (intercepted thiscall: the index,
/// the point, and an out-slot receiving the head of a node list where each
/// node holds an object pointer at `+0` and the next node at `+4`). Each
/// object carries a box as minimum xyz at `+0x00`, `+0x04`, `+0x08` and
/// maximum xyz at `+0x10`, `+0x14`, `+0x18`. For each node, every bound the
/// point violates contributes the miss marker from a second game global; the
/// six contributions are or-ed per axis, and the node matches when no axis
/// holds the marker (the marker is ordered false against everything only
/// when it is a NaN, which is the game's value; the file holds 0.0, under
/// which every node matches). Comparisons treat unordered (NaN) operands as
/// passing, exactly like the original's compare-and-branch pairs. The result
/// list is released (intercepted thiscall of no arguments) on every path,
/// and the matching object pointer, or 0, is returned.
///
/// Original: 0x00C72570 (cdecl, one stack argument; returns object or null).
lf_checker_rt::export!(cdecl, rw_00C72570(point: u32) -> u32 {
    unsafe {
        const INDEX_GLOBAL: u32 = 0x16DD680;
        const MISS_GLOBAL: u32 = 0x17AD148;
        const QUERY: u32 = 1;
        const RELEASE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read() }
        }
        /// Original `comiss a, b` followed by `jbe taken`: taken exactly
        /// when `a > b` is false, including unordered operands.
        #[inline(always)]
        fn below_or_equal(a: f32, b: f32) -> bool {
            !(core::hint::black_box(a) > core::hint::black_box(b))
        }

        let index = g32(INDEX_GLOBAL);
        if index == 0 {
            return 0;
        }
        let mut head: u32 = 0;
        let head_ptr = (&mut head as *mut u32) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(QUERY, u32, index, point, head_ptr);

        let px = rdf(point);
        let py = rdf(point + 4);
        let pz = rdf(point + 8);
        let miss = f32::from_bits(g32(MISS_GLOBAL));
        let mut node = head;
        let mut found = 0u32;
        while node != 0 {
            let obj = rd32(node);
            let x0 = if below_or_equal(rdf(obj), px) { 0.0f32 } else { miss };
            let y0 = if below_or_equal(rdf(obj + 4), py) { 0.0f32 } else { miss };
            let z0 = if below_or_equal(rdf(obj + 8), pz) { 0.0f32 } else { miss };
            let x1 = if below_or_equal(px, rdf(obj + 0x10)) { 0.0f32 } else { miss };
            let y1 = if below_or_equal(py, rdf(obj + 0x14)) { 0.0f32 } else { miss };
            let z1 = if below_or_equal(pz, rdf(obj + 0x18)) { 0.0f32 } else { miss };
            let ox = f32::from_bits(x0.to_bits() | x1.to_bits());
            let oy = f32::from_bits(y0.to_bits() | y1.to_bits());
            let oz = f32::from_bits(z0.to_bits() | z1.to_bits());
            if ox.is_nan() {
                node = rd32(node + 4);
                continue;
            }
            if oy.is_nan() {
                node = rd32(node + 4);
                continue;
            }
            if !oz.is_nan() {
                found = obj;
                break;
            }
            node = rd32(node + 4);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, head_ptr);
        found
    }
});
