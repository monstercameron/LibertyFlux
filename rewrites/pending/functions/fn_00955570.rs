// original: 0x00955570 resource_file_loader
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};
// --- fn 0x00955570 -----------------------------------------------------------
// Callee ids (see out/contracts/fn_00955570.json).
const L_ALLOC: u32 = 1; // 0x401210 cdecl/1: allocator, sequenced answers
const L_FILL: u32 = 2; // 0xDF9CC0 cdecl/3: fill (dst, 0, len)
const L_FIND_TAG: u32 = 3; // 0xDFC920 cdecl/2: tag search, 0 = append suffix
const L_OPEN: u32 = 4; // 0x8C4CF0 cdecl/2: open handle
const L_FREE: u32 = 5; // 0x401250 cdecl/1: release
const L_SIZE: u32 = 6; // 0x40EAB0 thiscall/0: payload size
const L_READ_HEAD: u32 = 7; // 0x40E870 thiscall/2: read stored word
const L_READ_BODY: u32 = 8; // 0x40E870 thiscall/2: read payload
const L_CHECKSUM: u32 = 9; // 0x8D4E20 cdecl/2: checksum over (buf, len)
const L_NEW: u32 = 10; // 0x433420 cdecl/8: build fallback object
const L_PARSE: u32 = 11; // 0x8F8540 cdecl/2: parse buffer
const L_SET: u32 = 12; // 0x4335E0 thiscall/1: attach parsed value
const L_CLOSE: u32 = 13; // 0x8C4650 cdecl/1: close handle
const L_MKOBJ: u32 = 14; // 0x40EA40 thiscall/1: build validated object
const L_FINAL: u32 = 15; // 0x6A2C40 cdecl/2: finalize handle

const TAG_SUFFIX: u32 = 0xE8AAB4; // suffix probe tag
const EXT_DWORD: u32 = 0xE8AABC; // appended suffix, first 4 bytes
const EXT_BYTE: u32 = 0xE8AAC0; // appended suffix, 5th byte
const TAG_OPEN3: u32 = 0xE8AAC4; // open tag

// Load a resource file and validate it, writing the result to `out`.
//
// Copies `path` (appending a suffix when the tag search misses), opens it,
// reads a stored checksum plus payload, and compares against a computed
// checksum. On match the handle is finalized into `out`; otherwise a
// fallback object is built and populated from the parsed buffer. Returns
// nonzero when an object was produced, 0 on every failure path.
export!(cdecl, rw_00955570(path: u32, out: u32) -> u32 {
    if path == 0 {
        return 0;
    }
    if unsafe { (path as *const u8).read() } == 0 {
        return 0;
    }
    if out == 0 {
        return 0;
    }
    unsafe { (out as *mut u32).write(0) };
    let mut len = 0usize;
    while unsafe { ((path as usize + len) as *const u8).read() } != 0 {
        len += 1;
    }
    let total = (len as u32).wrapping_add(8);
    let copied = callee_cdecl!(L_ALLOC, u32, total);
    if copied == 0 {
        return 0;
    }
    callee_cdecl!(L_FILL, u32, copied, 0, total);
    let mut i = 0usize;
    loop {
        let b = unsafe { ((path as usize + i) as *const u8).read() };
        unsafe { ((copied as usize + i) as *mut u8).write(b) };
        i += 1;
        if b == 0 {
            break;
        }
    }
    let found = callee_cdecl!(L_FIND_TAG, u32, copied, relocated(TAG_SUFFIX));
    if found == 0 {
        let ext4 = unsafe { (global::<u32>(EXT_DWORD) as *const u32).read() };
        unsafe { ((copied as usize + len) as *mut u32).write_unaligned(ext4) };
        let ext1 = unsafe { (global::<u8>(EXT_BYTE) as *const u8).read() };
        unsafe { ((copied as usize + len + 4) as *mut u8).write(ext1) };
    }
    let handle = callee_cdecl!(L_OPEN, u32, copied, relocated(TAG_OPEN3));
    if handle == 0 {
        callee_cdecl!(L_FREE, u32, copied);
        return 0;
    }
    let size = callee_thiscall!(L_SIZE, u32, handle);
    let body_len = size.wrapping_sub(4);
    let buf = callee_cdecl!(L_ALLOC, u32, body_len);
    if buf == 0 {
        callee_cdecl!(L_FREE, u32, copied);
        callee_cdecl!(L_CLOSE, u32, handle);
        return 0;
    }
    let mut stored = 0u32;
    callee_thiscall!(L_READ_HEAD, u32, handle, (&mut stored as *mut u32) as u32, 4);
    callee_thiscall!(L_READ_BODY, u32, handle, buf, body_len);
    let sum = callee_cdecl!(L_CHECKSUM, u32, buf, body_len);
    if stored != sum {
        let obj = callee_cdecl!(L_NEW, u32, 0x1c8, 0x100, 1, 1, 0, 0, 0, 0);
        unsafe { (out as *mut u32).write(obj) };
        if obj != 0 {
            let mut slot = buf;
            let parsed = callee_cdecl!(L_PARSE, u32, (&mut slot as *mut u32) as u32, 0x42);
            let inner = unsafe { (parsed as *const u32).read() };
            callee_thiscall!(L_SET, u32, obj, inner);
        }
        callee_cdecl!(L_CLOSE, u32, handle);
        callee_cdecl!(L_FREE, u32, copied);
        callee_cdecl!(L_FREE, u32, buf);
        (obj != 0) as u32
    } else {
        callee_thiscall!(L_MKOBJ, u32, handle, 4);
        let fin = callee_cdecl!(L_FINAL, u32, handle, 0);
        unsafe { (out as *mut u32).write(fin) };
        // Note: the original does not close the handle on this path.
        callee_cdecl!(L_FREE, u32, copied);
        callee_cdecl!(L_FREE, u32, buf);
        (fin != 0) as u32
    }
});
