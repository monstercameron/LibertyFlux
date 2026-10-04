// original: 0x00948790 point_box_query
/// Probe a point against a table-selected volume, then refine and commit.
///
/// Takes an object pointer, a pointer to three floats (x, y, z) and a
/// float scale. A global word selects an entry from the table at the same
/// global address; the entry plus 0x10 becomes the `this` pointer of a
/// scripted four-float probe helper called with (x, y, z, 4.0). A non-zero
/// probe answer returns 0. Otherwise a corner box of half-size 4.0 is
/// built around the point and its low corner, together with a callback
/// address, a zero out-word slot and the constants 0x1c and 0xd, goes to
/// a scripted five-word helper; a non-zero out-word returns 0. Then a
/// scripted six-word helper is called with (object, x, y, z, 1, 1), and
/// the scale times pi/180 goes through the first virtual slot (+0xc) of
/// the object with the object as `this`. Returns 1 on the full path.
/// The remaining corner words and one uninitialized frame word the
/// original shuffles around are dead stores: they reach no call, no
/// return and no memory the checker compares, so they are omitted.
export!(stdcall, rw_00948790(obj: u32, pt: u32, scale_arg: f32) -> u32 {
    unsafe {
        const PROBE_ID: u32 = 1;
        const REFINE_ID: u32 = 2;
        const COMMIT_ID: u32 = 3;
        const APPLY_ID: u32 = 4;
        const TABLE_GLOB: u32 = 0x118D818;
        const ENTRY_THIS_OFF: u32 = 0x10;
        const RADIUS_GLOB: u32 = 0xFE8AB8;
        const DEG2RAD_GLOB: u32 = 0xFE8728;
        const FOUR_BITS: u32 = 0x40800000;
        // The original pushes this code address as data; its push site is
        // relocated at load, so the rewrite must relocate it too.
        const CALLBACK_FILE_VA: u32 = 0x9498C0;
        const VTABLE_SLOT: u32 = 0xC;

        let x = *(pt as *const f32);
        let y = *((pt.wrapping_add(4)) as *const f32);
        let z = *((pt.wrapping_add(8)) as *const f32);
        let idx = *(global::<u32>(TABLE_GLOB));
        let entry = *(global::<u32>(
            TABLE_GLOB.wrapping_add(idx.wrapping_mul(4)),
        ));
        let probe: u32 = callee_thiscall!(
            PROBE_ID,
            u32,
            entry.wrapping_add(ENTRY_THIS_OFF),
            x.to_bits(),
            y.to_bits(),
            z.to_bits(),
            FOUR_BITS
        );
        if probe != 0 {
            return 0;
        }
        let c = *(relocated(RADIUS_GLOB) as *const f32);
        // Low corner of the box; the high corner and the middle words the
        // original also stores are never read back (see doc comment).
        let lo = [x - c, y - c, z - c];
        let mut out_word: u32 = probe;
        let _: u32 = callee_cdecl!(
            REFINE_ID,
            u32,
            lo.as_ptr() as u32,
            relocated(CALLBACK_FILE_VA),
            (&mut out_word as *mut u32) as u32,
            0x1c,
            0xd
        );
        if out_word != 0 {
            return 0;
        }
        let _: u32 = callee_cdecl!(
            COMMIT_ID,
            u32,
            obj,
            x.to_bits(),
            y.to_bits(),
            z.to_bits(),
            1,
            1
        );
        let scaled = scale_arg * *(relocated(DEG2RAD_GLOB) as *const f32);
        // Same indirection as the original: first virtual slot of obj.
        let vt = *(obj as *const u32);
        let slot: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(VTABLE_SLOT)) as *const u32));
        let _ = slot(obj, scaled.to_bits());
        1
    }
});
