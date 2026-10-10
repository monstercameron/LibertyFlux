use lf_checker_rt::{callee_thiscall, export, global};

#[inline(always)]
unsafe fn read_global_word(file_va: u32) -> u32 {
    unsafe { core::ptr::read_volatile(global::<u32>(file_va)) }
}

#[inline(always)]
unsafe fn write_indexed_word(base: u32, value: u32) {
    let index = unsafe { read_global_word(0x0115_F5F4) };
    let address = base.wrapping_add(index.wrapping_mul(4));
    unsafe { core::ptr::write_volatile(global::<u32>(address), value) };
}

/// Index-zero and index-one stage for the shared tail entered by status 0x27 or 0x26, or
/// when the guarded entry byte is nonzero. The contract cycles status values
/// 0x27, 0x26, and 0x25 with the entry byte varied so each reaches this tail;
/// it cycles the table index across zero and one, pins byte 0x1218493 to zero, and byte
/// 0x1218497 nonzero. It compares four helper calls and five table words;
/// frame-relative pointer values are skipped while both helper snapshots are
/// compared. Full stack-write comparison is outside this stage contract.
unsafe fn tail_stage(first_table_base: u32) {
    unsafe {
        let mut scratch = [0u32; 15];
        scratch[0] = read_global_word(0x0128_E310);
        scratch[1] = read_global_word(0x0128_E314);
        scratch[2] = read_global_word(0x0128_E318);
        scratch[4] = read_global_word(0x0128_E320);
        scratch[5] = read_global_word(0x0128_E324);
        scratch[6] = read_global_word(0x0128_E328);
        scratch[8] = read_global_word(0x0128_E330);
        scratch[9] = read_global_word(0x0128_E334);
        scratch[10] = read_global_word(0x0128_E338);
        scratch[12] = read_global_word(0x0128_E340);
        scratch[13] = read_global_word(0x0128_E344);
        scratch[14] = read_global_word(0x0128_E348);
        let out = scratch.as_mut_ptr() as u32;

        let _ = callee_thiscall!(
            18,
            u32,
            global::<u8>(0x0115_D9A0) as u32,
            out,
            0u32,
            0x3F80_0000u32,
        );
        let _ = callee_thiscall!(
            19,
            u32,
            global::<u8>(0x0115_D9A0) as u32,
            out,
            0u32,
            0x3F80_0000u32,
        );

        write_indexed_word(first_table_base, 0x3F80_0000);
        write_indexed_word(0x0115_F480, 0);
        write_indexed_word(0x0115_F490, 0);
        write_indexed_word(0x0115_F4A0, 0);
        write_indexed_word(0x0115_F4B0, 0);

        if core::ptr::read_volatile(global::<u8>(0x0121_8493)) == 0 {
            let _ = callee_thiscall!(
                8,
                u32,
                global::<u8>(0x0115_DEF0) as u32,
                0x3F34_FDF4u32,
                0xBF34_FDF4u32,
                0u32,
            );
        }
    }
}

/// On the stage path, call the scripted status helper, then mirror the
/// shared tail's source-word snapshots, calls, and five index-scaled writes.
/// The strict contract replays table indices zero and one and bounds the flags to this branch.
#[allow(unused_doc_comments)]
export!(thiscall, rw_0096d490(_this: u32) -> () {
    unsafe {
        let status = callee_thiscall!(
            1,
            u32,
            global::<u8>(0x0128_E310) as u32,
            0xFFFF_FFFFu32,
        );
        if status == 0x27 || status == 0x26 || core::ptr::read_volatile(global::<u8>(0x0121_8492)) != 0 {
            tail_stage(0x0115_F470);
        }
    }
});

/// Same stage with one deliberate fault at index one: the first table value
/// is written one dword early only for that index. Index zero remains correct
/// so the mutant must fail specifically on the newly covered output word.
#[allow(unused_doc_comments)]
export!(thiscall, mut_0096d490(_this: u32) -> () {
    unsafe {
        let status = callee_thiscall!(
            1,
            u32,
            global::<u8>(0x0128_E310) as u32,
            0xFFFF_FFFFu32,
        );
        if status == 0x27 || status == 0x26 || core::ptr::read_volatile(global::<u8>(0x0121_8492)) != 0 {
            let first_base = if read_global_word(0x0115_F5F4) == 1 {
                0x0115_F46C
            } else {
                0x0115_F470
            };
            tail_stage(first_base);
        }
    }
});





