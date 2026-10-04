// original: 0x00df68e0 saved_clip_name_lookup
/// Report whether a clip name is listed in the saved-clips search.
/// Opens a search over the "Videos\Clips" folder keyed by the
/// "MyFavorites.dat" name held in globals, reads each 32-byte candidate
/// name in turn, and compares it with the given NUL-terminated name.
/// Returns 1 on the first exact match, 0 when the search handle fails to
/// open or the entries run out. The search is always closed before
/// returning, on every path.
export!(stdcall, rw_00df68e0(name: u32) -> u32 {
    unsafe {
        /// Folder the search is restricted to (global string).
        const CLIPS_DIR: u32 = 0x00F01ECC;
        /// Search key passed alongside the candidate buffer (global data).
        const SEARCH_KEY_ARG: u32 = 0x00F01EDC;
        /// First half of the wanted file name (global string).
        const WANT_NAME_LO: u32 = 0x00F01EBC;
        /// Second half of the wanted file name (global string).
        const WANT_NAME_HI: u32 = 0x00F01EC4;
        /// Candidate entry buffer length in bytes.
        const ENTRY_LEN: u32 = 0x20;

        let want_lo = *(global::<u64>(WANT_NAME_LO));
        let want_hi = *(global::<u64>(WANT_NAME_HI));
        // Original pushes 0 then the folder, so the callee sees (folder, 0).
        let _: u32 = callee_cdecl!(1, u32, relocated(CLIPS_DIR), 0);
        let mut key = [want_lo, want_hi];
        let handle: u32 = callee_cdecl!(
            2,
            u32,
            (&mut key as *mut [u64; 2]) as u32,
            relocated(SEARCH_KEY_ARG)
        );
        if handle == 0 {
            let _: u32 = callee_cdecl!(5, u32,);
            return 0;
        }
        // The candidate buffer is cleared once and reused across entries:
        // each fill call observes the previous entry's leftover bytes.
        let mut entry = [0u32; 8];
        loop {
            let more: u32 = callee_cdecl!(
                3,
                u32,
                handle,
                entry.as_mut_ptr() as u32,
                ENTRY_LEN
            );
            if more == 0 {
                let _: u32 = callee_cdecl!(4, u32, handle);
                let _: u32 = callee_cdecl!(5, u32,);
                return 0;
            }
            let bytes = entry.as_ptr() as *const u8;
            let mut i = 0u32;
            let mut equal = true;
            loop {
                let want = *((name.wrapping_add(i)) as *const u8);
                let got = *bytes.add(i as usize);
                if want != got {
                    equal = false;
                    break;
                }
                if want == 0 {
                    break;
                }
                i = i.wrapping_add(1);
            }
            if equal {
                let _: u32 = callee_cdecl!(4, u32, handle);
                let _: u32 = callee_cdecl!(5, u32,);
                return 1;
            }
        }
    }
});
