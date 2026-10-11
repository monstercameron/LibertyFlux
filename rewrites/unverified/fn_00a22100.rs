// original: 0x00A22100 ped_task_initialize_phase_globals

/// Initialize phase-output globals with fixed words and three four-word tables.
///
/// The no-argument cdecl routine copies one uninitialized stack word into twelve
/// writable float slots. The contract pins that stack fill to zero because the
/// original does not initialize or otherwise define the source word. It then
/// copies three four-word constants from read-only data and writes twelve
/// triplets of fixed 32-bit words to the game's global data. All global writes
/// are word-sized and retain their original order. The routine has no defined
/// return value; its EAX contents are not part of the contract.
lf_checker_rt::export!(cdecl, rw_00A22100() -> () {
    unsafe {
        const FLOAT_WORD_TARGETS: [u32; 12] = [
            0x012D_D3AC, 0x012D_D3BC, 0x012D_D3DC,
            0x012D_D44C, 0x012D_D45C, 0x012D_D47C,
            0x012D_D4EC, 0x012D_D4FC, 0x012D_D51C,
            0x012D_D58C, 0x012D_D59C, 0x012D_D5BC,
        ];
        const VECTOR_COPIES: [(u32, u32); 4] = [
            (0x00E9_B520, 0x012D_D3E0),
            (0x00E9_B530, 0x012D_D480),
            (0x00E9_B540, 0x012D_D520),
            (0x00E9_B550, 0x012D_D5C0),
        ];
        const RATE_WORD_A: u32 = 0x38D1_B717;
        const RATE_WORD_B: u32 = 0x3727_C5AC;
        const ZERO_WORD: u32 = 0;
        const SCALE_WORD_A: u32 = 0x3FF3_3333;
        const SCALE_WORD_B: u32 = 0x3D75_C28F;
        const SCALE_WORD_C: u32 = 0x3AC4_9BA6;
        const SCALE_WORD_D: u32 = 0x3FD9_999A;
        const SCALE_WORD_E: u32 = 0x3D8F_5C29;
        const SCALE_WORD_F: u32 = 0x3B03_126F;
        const SCALE_WORD_G: u32 = 0x3FA6_6666;
        const SCALE_WORD_H: u32 = 0x3D99_999A;
        const SCALE_WORD_I: u32 = 0x3B44_9BA6;
        const SCALE_WORD_J: u32 = 0x3F8C_CCCD;
        const PRESET_WRITES: [(u32, u32); 36] = [
            (0x012D_D3A0, RATE_WORD_A), (0x012D_D3A4, RATE_WORD_B),
            (0x012D_D3A8, RATE_WORD_B),
            (0x012D_D3B0, ZERO_WORD), (0x012D_D3B4, ZERO_WORD),
            (0x012D_D3B8, ZERO_WORD),
            (0x012D_D3D0, SCALE_WORD_A), (0x012D_D3D4, SCALE_WORD_A),
            (0x012D_D3D8, SCALE_WORD_A),
            (0x012D_D440, SCALE_WORD_B), (0x012D_D444, SCALE_WORD_B),
            (0x012D_D448, SCALE_WORD_B),
            (0x012D_D450, SCALE_WORD_C), (0x012D_D454, SCALE_WORD_C),
            (0x012D_D458, SCALE_WORD_C),
            (0x012D_D470, SCALE_WORD_D), (0x012D_D474, SCALE_WORD_D),
            (0x012D_D478, SCALE_WORD_D),
            (0x012D_D4E0, SCALE_WORD_E), (0x012D_D4E4, SCALE_WORD_E),
            (0x012D_D4E8, SCALE_WORD_E),
            (0x012D_D4F0, SCALE_WORD_F), (0x012D_D4F4, SCALE_WORD_F),
            (0x012D_D4F8, SCALE_WORD_F),
            (0x012D_D510, SCALE_WORD_G), (0x012D_D514, SCALE_WORD_G),
            (0x012D_D518, SCALE_WORD_G),
            (0x012D_D580, SCALE_WORD_H), (0x012D_D584, SCALE_WORD_H),
            (0x012D_D588, SCALE_WORD_H),
            (0x012D_D590, SCALE_WORD_I), (0x012D_D594, SCALE_WORD_I),
            (0x012D_D598, SCALE_WORD_I),
            (0x012D_D5B0, SCALE_WORD_J), (0x012D_D5B4, SCALE_WORD_J),
            (0x012D_D5B8, SCALE_WORD_J),
        ];

        #[inline(always)]
        unsafe fn write_global(file_va: u32, value: u32) {
            unsafe { lf_checker_rt::global::<u32>(file_va).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn read_global(file_va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
        }

        // The checker fixes this source to zero; all twelve writes preserve its bits.
        for destination in FLOAT_WORD_TARGETS {
            write_global(destination, 0);
        }
        for (source, destination) in VECTOR_COPIES {
            let mut word = 0;
            while word < 4 {
                let value = read_global(source + word * 4);
                write_global(destination + word * 4, value);
                word += 1;
            }
        }
        for (destination, value) in PRESET_WRITES {
            write_global(destination, value);
        }
    }
});
