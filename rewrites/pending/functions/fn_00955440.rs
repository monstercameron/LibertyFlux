// original: 0x00955440 resource_header_float
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};
// --- fn 0x00955440 -----------------------------------------------------------
// Callee ids (see out/contracts/fn_00955440.json).
const RESOLVE_STATE: u32 = 1; // 0x5A3240 cdecl/1: state token from global state
const MATCH_RESOURCE: u32 = 2; // 0xDF9E88 cdecl/2: key/token matcher, 0 = proceed
const LOG_LINE: u32 = 3; // 0x8C5220 cdecl/1: log one line
const LOG_FALLBACK: u32 = 4; // 0x8C54D0 cdecl/2: fallback log path
const OPEN_HANDLE: u32 = 5; // 0x8C4CF0 cdecl/2: open handle for (key, tag)
const READ_HEADER: u32 = 6; // 0x8C4E50 cdecl/3: read bytes into caller's buffer
const CLOSE_HANDLE: u32 = 7; // 0x8C4650 cdecl/1: close handle

const G_STATE: u32 = 0x11D6FD4; // dword drive state, re-read for the switch
const G_READY: u32 = 0x18B6E8D; // byte gate for the resolve-and-log prologue

const TAG_STATE_A: u32 = 0xE8AE2C; // log tag when state == 0
const TAG_STATE_B: u32 = 0xE8AE34; // log tag when state == 1
const TAG_STATE_C: u32 = 0xE8AE40; // log tag when state == 2
const TAG_FALLBACK: u32 = 0xE8AE4C; // log tag on the fallback path
const TAG_OPEN: u32 = 0xE8AE5C; // open tag
const TAG_OK: u32 = 0xE8926B; // log tag after a good header
const TAG_BAD_MAGIC: u32 = 0xE8926A; // log tag after a bad header
const TAG_NO_HANDLE: u32 = 0xE89292; // log tag when open fails

const MAGIC0: u32 = 0x52504C59; // header magic, first dword
const MAGIC1: u32 = 0x52334850; // header magic, second dword
const HEADER_WORDS: usize = 5; // 0x14 bytes read into the frame buffer
const VALUE_WORD: usize = 4; // float result lives at buffer offset 0x10

// Read a resource header and return the float it carries.
//
// Opens `key`, reads a 5-word header, and returns the word at offset 0x10
// as a float when the 8-byte magic matches. Any failure (gate closed,
// matcher veto, open failure, bad magic) logs and yields 0.0.
export!(cdecl, rw_00955440(key: u32) -> f32 {
    let gate: u8 = unsafe { (global::<u8>(G_READY) as *const u8).read() };
    if gate == 0 {
        callee_cdecl!(LOG_FALLBACK, u32, relocated(TAG_FALLBACK), 0);
    } else {
        let state: u32 = unsafe { (global::<u32>(G_STATE) as *const u32).read() };
        let token = callee_cdecl!(RESOLVE_STATE, u32, state);
        let veto = callee_cdecl!(MATCH_RESOURCE, u32, key, token);
        if veto != 0 {
            callee_cdecl!(LOG_FALLBACK, u32, relocated(TAG_FALLBACK), 0);
        } else {
            // The original re-reads the state global for this switch.
            let state2: u32 = unsafe { (global::<u32>(G_STATE) as *const u32).read() };
            match state2 {
                0 => { callee_cdecl!(LOG_LINE, u32, relocated(TAG_STATE_A)); }
                1 => { callee_cdecl!(LOG_LINE, u32, relocated(TAG_STATE_B)); }
                2 => { callee_cdecl!(LOG_LINE, u32, relocated(TAG_STATE_C)); }
                _ => {}
            }
        }
    }
    let handle = callee_cdecl!(OPEN_HANDLE, u32, key, relocated(TAG_OPEN));
    if handle == 0 {
        callee_cdecl!(LOG_LINE, u32, relocated(TAG_NO_HANDLE));
        return 0.0;
    }
    let mut header = [0u32; HEADER_WORDS];
    callee_cdecl!(READ_HEADER, u32, handle, header.as_mut_ptr() as u32, 0x14);
    if header[0] == MAGIC0 && header[1] == MAGIC1 {
        callee_cdecl!(CLOSE_HANDLE, u32, handle);
        callee_cdecl!(LOG_LINE, u32, relocated(TAG_OK));
        f32::from_bits(header[VALUE_WORD])
    } else {
        callee_cdecl!(CLOSE_HANDLE, u32, handle);
        callee_cdecl!(LOG_LINE, u32, relocated(TAG_BAD_MAGIC));
        0.0
    }
});
