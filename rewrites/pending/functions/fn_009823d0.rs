// original: 0x009823d0 audio_voice_register_by_hash
use lf_k2_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global, relocated};

/// Audio voice registration by hashed name.
///
/// After the same window/mute/busy gates as its sibling allocator (plus a
/// master-audio-enable byte), this resolves the voice handle carried by the
/// argument object: it chases two pointers, hashes the resulting name with
/// the string hasher, and looks the hash up in the global voice bank. A hit
/// builds a short-lived key object on the stack from this object's key
/// material, scans the 245-entry registration table for the first free slot,
/// fills that slot (owner handle, state word `0x0101`, generation `-1`,
/// flags `0`, live byte `1`) and tears the key object down. A lookup miss,
/// a full table, or any gate failure exits without writing.
export!(thiscall, rw_009823d0(this: u32, handle_obj: u32) -> () {
    const TABLE_BASE_OFF: u32 = 0x5bfc;
    const TABLE_LIVE_OFF: u32 = 0x5c0c;
    const ENTRY_SIZE: u32 = 20;
    const MAX_VOICES: u32 = 245;
    unsafe {
        // Same window/mute gates as the sibling allocator.
        let hwnd = *global::<u32>(0x17accd8);
        let iconic = callee_stdcall!(1, u32, hwnd);
        let blocked = if iconic == 0 {
            *global::<u8>(0x105b48f) != 0 && *global::<u8>(0x17ed8d1) != 0
        } else {
            true
        };
        let muted = *global::<u8>(0x1173590) | *global::<u8>(0x1173591);
        if blocked || muted != 0 {
            return;
        }
        // Busy probe (low byte only; incoming ECX is call-clobbered leftover).
        if (callee_cdecl!(2, u32,) & 0xFF) != 0 {
            return;
        }
        // Master audio enable.
        if *global::<u8>(0x1038a20) == 0 {
            return;
        }
        // Chase handle_obj+0x78 -> +0xf8 (null-tolerant), then hash the name.
        let inner = *((handle_obj as *const u32).byte_add(0x78));
        let name = if inner == 0 {
            0
        } else {
            *((inner as *const u32).byte_add(0xf8))
        };
        let hash = callee_cdecl!(3, u32, name, 0);
        // Look the hash up in the global voice bank (fixed image object).
        let bank = relocated(0x115dc18);
        if callee_thiscall!(4, u32, bank, hash) == 0 {
            return;
        }
        // Build the short-lived key from this object's key material. The
        // original places it in its own stack frame; the stub neither reads
        // nor writes anything the function later observes, so any scratch
        // address works (the contract skips the frame address).
        let mut key = [0u32; 2];
        let key_ptr = key.as_mut_ptr() as u32;
        let material = this.wrapping_add(0x6f3c);
        callee_thiscall!(5, u32, key_ptr, material);
        // Scan for the first free registration entry.
        let mut index = 0u32;
        loop {
            let live = this.wrapping_add(TABLE_LIVE_OFF).wrapping_add(index.wrapping_mul(ENTRY_SIZE));
            if *(live as *const u8) == 0 {
                break;
            }
            index = index.wrapping_add(1);
            if index >= MAX_VOICES {
                callee_thiscall!(6, u32, key_ptr);
                return;
            }
        }
        let entry = this
            .wrapping_add(TABLE_BASE_OFF)
            .wrapping_add(index.wrapping_mul(ENTRY_SIZE));
        *((entry as *mut u32).byte_add(0xc)) = handle_obj;
        *((entry as *mut u16).byte_add(0x10)) = 0x0101;
        *((entry as *mut u32).byte_add(8)) = 0xffff_ffff;
        *(entry as *mut u32) = 0;
        *((entry as *mut u8).byte_add(0x12)) = 1;
        callee_thiscall!(6, u32, key_ptr);
    }
});
