// original: 0x008d5ac0 dispatch_indexed_tris
/// Dispatches indexed triangle batches through fixed helper calls, then chains to a shared tail routine.
///
/// Takes four stack words: `verts` (pointer to a vertex array), `count`
/// (signed batch count), `obj` (object pointer or null), and `param` (a
/// 32-bit parameter carried verbatim into every batch call; only its bits
/// are used, never its value as a float).
///
/// It first calls helper 1 with 0. It then selects a path: path A when
/// `obj` is non-null and the flag byte at `obj + 3` is non-zero, path B
/// otherwise.
///
/// Path A calls helper 2 with (3, 3 * count). Path B calls helper 5 with
/// (7, 0), (5, 1) and (15, 0), then helper 2 with (3, 3 * count). When
/// `count` is zero or negative the function chains to the tail routine
/// immediately with (`verts`, `count`, `obj`, `param`).
///
/// Otherwise it runs `count` iterations. Each iteration calls helper 3
/// three times with successive 8-byte-spaced pointers derived from the
/// current vertex pointer (helper 3 refreshes two words at each pointer;
/// the stub models those stores), then calls helper 4 three times, each
/// with seven words: two refreshed vertex words, `param`, three words
/// from the coefficient table, and a selector word. On path A the
/// selector is the word at `obj` (also stored to the shared slot each
/// iteration) and the vertex pointer advances by 24 with its trailing
/// value kept in the incoming `obj` argument slot; on path B the
/// selector is the shared slot's current value and the vertex pointer
/// advances by 24 from `verts + 8`.
///
/// Every helper answer is ignored; control flow depends only on `obj`,
/// its flag byte, and `count`. The function never returns normally: it
/// always ends by chaining to the tail routine (helper 6) with
/// (`verts`, `count`, slot, `param`), where slot is the trailing vertex
/// value on path A iterations and the pristine `obj` otherwise, and the
/// tail routine's answer is the function's answer. All multiplications
/// and pointer steps wrap modulo 2^32.
export!(cdecl, rw_008d5ac0(verts: u32, count: u32, obj: u32, param: u32) -> u32 {
    /// Coefficient table words read into every batch call (two neighbouring
    /// table words are stored to scratch and never read back, so they are
    /// not loaded).
    const COEF0: u32 = 0x017f59ec;
    /// Fourth coefficient word.
    const COEF3: u32 = 0x017f59f8;
    /// Fifth coefficient word.
    const COEF4: u32 = 0x017f59fc;
    /// Shared selector slot: written on path A, read on path B.
    const SHARED_SLOT: u32 = 0x0110dfb0;
    /// Stride between the three per-iteration helper-3 pointers.
    const PTR_STRIDE: u32 = 8;
    /// Vertex pointer advance per iteration.
    const VERT_ADVANCE: u32 = 0x18;
    unsafe {
        let _: u32 = callee_cdecl!(1, u32, 0u32);
        let path_a = obj != 0 && *((obj.wrapping_add(3)) as *const u8) != 0;
        if path_a {
            let _: u32 = callee_cdecl!(2, u32, 3u32, count.wrapping_mul(3));
            if (count as i32) <= 0 {
                return callee_cdecl!(6, u32, verts, count, obj, param);
            }
            let mut slot = verts.wrapping_add(PTR_STRIDE);
            let mut remaining = count;
            loop {
                let base = slot.wrapping_sub(PTR_STRIDE);
                let _: u32 = callee_cdecl!(3, u32, base);
                let mid = slot;
                let _: u32 = callee_cdecl!(3, u32, mid);
                let high = mid.wrapping_add(PTR_STRIDE);
                let _: u32 = callee_cdecl!(3, u32, high);
                let sel = *(obj as *const u32);
                *global::<u32>(SHARED_SLOT) = sel;
                let c0 = *global::<u32>(COEF0);
                let c3 = *global::<u32>(COEF3);
                let c4 = *global::<u32>(COEF4);
                let _: u32 = callee_cdecl!(
                    4, u32,
                    *(base as *const u32),
                    *((base.wrapping_add(4)) as *const u32),
                    param, c0, c3, c4, sel
                );
                let reloaded = slot;
                let _: u32 = callee_cdecl!(
                    4, u32,
                    *(reloaded as *const u32),
                    *((reloaded.wrapping_add(4)) as *const u32),
                    param, c0, c3, c4, sel
                );
                let _: u32 = callee_cdecl!(
                    4, u32,
                    *(high as *const u32),
                    *((reloaded.wrapping_add(12)) as *const u32),
                    param, c0, c3, c4, sel
                );
                slot = reloaded.wrapping_add(VERT_ADVANCE);
                remaining = remaining.wrapping_sub(1);
                if remaining == 0 {
                    break;
                }
            }
            callee_cdecl!(6, u32, verts, count, slot, param)
        } else {
            let _: u32 = callee_cdecl!(5, u32, 7u32, 0u32);
            let _: u32 = callee_cdecl!(5, u32, 5u32, 1u32);
            let _: u32 = callee_cdecl!(5, u32, 15u32, 0u32);
            let _: u32 = callee_cdecl!(2, u32, 3u32, count.wrapping_mul(3));
            if (count as i32) <= 0 {
                return callee_cdecl!(6, u32, verts, count, obj, param);
            }
            let mut cur = verts.wrapping_add(PTR_STRIDE);
            let mut remaining = count;
            loop {
                let base = cur.wrapping_sub(PTR_STRIDE);
                let _: u32 = callee_cdecl!(3, u32, base);
                let _: u32 = callee_cdecl!(3, u32, cur);
                let high = cur.wrapping_add(PTR_STRIDE);
                let _: u32 = callee_cdecl!(3, u32, high);
                let sel = *global::<u32>(SHARED_SLOT);
                let c0 = *global::<u32>(COEF0);
                let c3 = *global::<u32>(COEF3);
                let c4 = *global::<u32>(COEF4);
                let _: u32 = callee_cdecl!(
                    4, u32,
                    *(base as *const u32),
                    *((cur.wrapping_sub(4)) as *const u32),
                    param, c0, c3, c4, sel
                );
                let _: u32 = callee_cdecl!(
                    4, u32,
                    *(cur as *const u32),
                    *((cur.wrapping_add(4)) as *const u32),
                    param, c0, c3, c4, sel
                );
                let _: u32 = callee_cdecl!(
                    4, u32,
                    *(high as *const u32),
                    *((cur.wrapping_add(12)) as *const u32),
                    param, c0, c3, c4, sel
                );
                cur = cur.wrapping_add(VERT_ADVANCE);
                remaining = remaining.wrapping_sub(1);
                if remaining == 0 {
                    break;
                }
            }
            callee_cdecl!(6, u32, verts, count, obj, param)
        }
    }
});
