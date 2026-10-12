// original: 0x00A8F910 interior_snap_verts_to_corners

/// Compare four vertex records with four scripted corner records and copy a
/// complete corner record into each vertex whose squared distance is strictly
/// below the threshold. The two stack arguments are the source point and the
/// entity; its mesh pointer is at `+0xb4`, its material word at `+0xb8`, the
/// mesh vertex pointer at `+0x80`, and each vertex record is 16 bytes within a
/// 96-byte group. X, Y, and Z are subtracted in that order, squared, summed as
/// Y²+X² then +Z², and compared with the 0.5625 threshold using ordered `>`.
/// The local corner pointer is skipped at the first helper call and all 16
/// words are snapshotted. The function uses stdcall with two
/// stack words. The return register is not part of this routine's output.
lf_checker_rt::export!(stdcall, rw_00a8f910(source_point: u32, entity: u32) -> u32 {
    unsafe {
        const MESH_OFFSET: u32 = 0xb4;
        const MATERIAL_OFFSET: u32 = 0xb8;
        const VERTEX_BASE_OFFSET: u32 = 0x80;
        const VERTEX_GROUP_BYTES: u32 = 0x60;
        const VERTEX_RECORD_BYTES: u32 = 0x10;
        const COOKIE_GLOBAL: u32 = 0x0105_7fb4;
        const DISTANCE_THRESHOLD: u32 = 0x00fe_8848;

        let mesh = ((entity + MESH_OFFSET) as *const u32).read_unaligned();
        let material = ((entity + MATERIAL_OFFSET) as *const u32).read_unaligned();
        let mut corner_words = [0u32; 16];
        let corner_pointer = core::ptr::addr_of_mut!(corner_words) as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, 0, source_point, corner_pointer);
        let group_index: u32 = lf_checker_rt::callee_thiscall!(2, u32, mesh, 0, material);
        let vertex_base = ((mesh + VERTEX_BASE_OFFSET) as *const u32).read_unaligned();
        let threshold_bits = lf_checker_rt::global::<u32>(DISTANCE_THRESHOLD).read_unaligned();
        let threshold = f32::from_bits(threshold_bits);

        let mut record_index = 0u32;
        while record_index < 4 {
            let corner_word = (record_index * 4) as usize;
            let target_x = f32::from_bits(corner_words[corner_word]);
            let target_y = f32::from_bits(corner_words[corner_word + 1]);
            let target_z = f32::from_bits(corner_words[corner_word + 2]);
            let record = vertex_base
                .wrapping_add(group_index.wrapping_mul(VERTEX_GROUP_BYTES))
                .wrapping_add(record_index.wrapping_mul(VERTEX_RECORD_BYTES));
            let vertex_x = f32::from_bits((record as *const u32).read_unaligned());
            let vertex_y = f32::from_bits(((record + 4) as *const u32).read_unaligned());
            let vertex_z = f32::from_bits(((record + 8) as *const u32).read_unaligned());

            let dx = core::hint::black_box(vertex_x) - core::hint::black_box(target_x);
            let dy = core::hint::black_box(vertex_y) - core::hint::black_box(target_y);
            let dz = core::hint::black_box(vertex_z) - core::hint::black_box(target_z);
            let dx_squared = core::hint::black_box(dx) * core::hint::black_box(dx);
            let dy_squared = core::hint::black_box(dy) * core::hint::black_box(dy);
            let dz_squared = core::hint::black_box(dz) * core::hint::black_box(dz);
            let xy_squared = core::hint::black_box(dy_squared) + core::hint::black_box(dx_squared);
            let distance_squared = core::hint::black_box(xy_squared) + core::hint::black_box(dz_squared);

            if threshold > distance_squared {
                let destination = record as *mut u32;
                for word in 0..4 {
                    destination.add(word).write_unaligned(corner_words[corner_word + word]);
                }
            }
            record_index += 1;
        }

        let cookie = lf_checker_rt::global::<u32>(COOKIE_GLOBAL).read_unaligned();
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, cookie);
        let _ = threshold_bits;
        0
    }
});
