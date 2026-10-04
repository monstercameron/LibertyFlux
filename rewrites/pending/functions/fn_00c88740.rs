// original: 0x00c88740 audio_bounds_refresh
//! Audio bounds refresh: copy the two hook-provided records, then rebuild the
//! min/max corners over the live entry set when the state gate passes.
//!
//! `obj` carries both the vtable (hooks at slots 0x60/0x64) and the state
//! fields (mode bits at +0x28, state word at +0x1304, inner object at +0xDC4).
//! `out_min`/`out_max` are 16-byte outputs: three float corners plus a tag
//! dword supplied by the hooks. Returns a state word, or the entry-table
//! cursor when the gate passes.
export!(stdcall, rw_00c88740(obj: *mut u8, out_min: *mut u32, out_max: *mut u32) -> u32 {
    unsafe {
        let vtable = *(obj as *const u32);
        let hook_min: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vtable + 0x60) as *const u32));
        let rec = hook_min(obj as u32);
        copy_record(out_min, rec);
        let hook_max: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vtable + 0x64) as *const u32));
        let rec = hook_max(obj as u32);
        copy_record(out_max, rec);

        let mode = *(obj.add(0x28) as *const u32) & 0x3c0;
        if mode != 0x80 {
            return mode;
        }
        if *(obj.add(0x1304) as *const u32) != 4 {
            return mode;
        }
        let inner = *(obj.add(0xdc4) as *const u32);
        let cursor: u32 = callee_thiscall!(3, u32, inner);
        let aux = *((cursor + 0xec) as *const u32);

        // Reseed: min corners to +1e8, max corners to -1e8. The tag dwords
        // (word 3 of each output) keep the hook-provided values.
        const POS_SEED: u32 = 0x4cbebc20; // 1e8f
        const NEG_SEED: u32 = 0xccbebc20; // -1e8f
        *out_min = POS_SEED;
        *out_min.add(1) = POS_SEED;
        *out_min.add(2) = POS_SEED;
        *out_max = NEG_SEED;
        *out_max.add(1) = NEG_SEED;
        *out_max.add(2) = NEG_SEED;

        let live = *((cursor + 0x1f3) as *const u8) as u32;
        if live == 0 {
            return cursor;
        }
        let gate_table = *((cursor + 0xd4) as *const u32);
        let entry_table = *((aux + 0x80) as *const u32);
        let matrix_base = *((aux + 0x84) as *const u32);
        let mut idx: u32 = 0;
        loop {
            let gate = *((gate_table + idx * 4) as *const u32);
            if *((gate + 0xc) as *const u8) == 0 {
                let entry = *((entry_table + idx * 4) as *const u32);
                if entry != 0 {
                    let mat = matrix_base + idx * 0x40;
                    fold_min(out_min, entry, mat);
                    fold_max(out_max, entry, mat);
                }
            }
            idx += 1;
            // The original re-reads the count each pass; nothing in the loop
            // writes it, so this ends exactly when idx reaches the count.
            if idx >= *((cursor + 0x1f3) as *const u8) as u32 {
                break;
            }
        }
        cursor
    }
});

/// Copy one 16-byte hook record (tag, two floats, tag) to an output.
#[inline(always)]
unsafe fn copy_record(out: *mut u32, rec: u32) {
    *out = *(rec as *const u32);
    *out.add(1) = *((rec + 4) as *const u32);
    *out.add(2) = *((rec + 8) as *const u32);
    *out.add(3) = *((rec + 12) as *const u32);
}

/// Read one matrix row-dot in the original's accumulation order:
/// ((m[y] * v.y + m[x] * v.x) + m[z] * v.z) + m[w].
#[inline(always)]
unsafe fn dot_ordered(mat: u32, vi: u32, x: f32, y: f32, z: f32) -> f32 {
    let mx = f32::from_bits(*((mat + vi) as *const u32));
    let my = f32::from_bits(*((mat + vi + 0x10) as *const u32));
    let mz = f32::from_bits(*((mat + vi + 0x20) as *const u32));
    let mw = f32::from_bits(*((mat + vi + 0x30) as *const u32));
    ((my * y + mx * x) + mz * z) + mw
}

/// Transform the entry's first corner and fold it into the running minimum.
/// Keeps the old corner unless it is strictly greater (NaN-safe: an unordered
/// comparison keeps the old value, matching comiss+ja).
#[inline(always)]
unsafe fn fold_min(out: *mut u32, entry: u32, mat: u32) {
    let x = f32::from_bits(*((entry + 0x20) as *const u32));
    let y = f32::from_bits(*((entry + 0x24) as *const u32));
    let z = f32::from_bits(*((entry + 0x28) as *const u32));
    let tx = dot_ordered(mat, 0, x, y, z);
    let ty = dot_ordered(mat, 4, x, y, z);
    let tz = dot_ordered(mat, 8, x, y, z);
    let o0 = f32::from_bits(*out);
    *out = (if o0 > tx { tx } else { o0 }).to_bits();
    let o1 = f32::from_bits(*out.add(1));
    *out.add(1) = (if o1 > ty { ty } else { o1 }).to_bits();
    let o2 = f32::from_bits(*out.add(2));
    *out.add(2) = (if o2 > tz { tz } else { o2 }).to_bits();
}

/// Transform the entry's second corner and fold it into the running maximum.
/// Keeps the old corner unless the new one is strictly greater.
#[inline(always)]
unsafe fn fold_max(out: *mut u32, entry: u32, mat: u32) {
    let x = f32::from_bits(*((entry + 0x10) as *const u32));
    let y = f32::from_bits(*((entry + 0x14) as *const u32));
    let z = f32::from_bits(*((entry + 0x18) as *const u32));
    let tx = dot_ordered(mat, 0, x, y, z);
    let ty = dot_ordered(mat, 4, x, y, z);
    let tz = dot_ordered(mat, 8, x, y, z);
    let o0 = f32::from_bits(*out);
    *out = (if tx > o0 { tx } else { o0 }).to_bits();
    let o1 = f32::from_bits(*out.add(1));
    *out.add(1) = (if ty > o1 { ty } else { o1 }).to_bits();
    let o2 = f32::from_bits(*out.add(2));
    *out.add(2) = (if tz > o2 { tz } else { o2 }).to_bits();
}
