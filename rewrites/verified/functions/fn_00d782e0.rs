// original: 0x00D782E0 CRenderPhaseWarpShadow::vf3

/// Render the warp-shadow phase when its table row selects it.
///
/// Indexes three global rows by `[this + 0x940] * 0x110`: the enable byte
/// must be 1 (else returns the scaled index), the state dword must be 0
/// or 2 (else returns it), and the mode dword 5, 3 or 4 stores 0x101, 1
/// or 0x100 to the word at `+0x15` (other modes store nothing). Then
/// issues a six-argument draw, queries the device into `+0x938`, commits
/// the pair, and tail-jumps to the epilogue, whose answer is returned.
/// Thiscall: object in `ecx`, no stack words.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const DRAW: u32 = 1;
const QUERY: u32 = 2;
const COMMIT: u32 = 3;
const EPILOGUE: u32 = 4;

export!(thiscall, rw_00d782e0(this: u32) -> u32 {
    unsafe {
        const INDEX_OFF: u32 = 0x940;
        const ROW_STRIDE: u32 = 0x110;
        const ENABLE_ROW: u32 = 0x0119f1ed;
        const STATE_ROW: u32 = 0x0119f100;
        const MODE_ROW: u32 = 0x0119f1fc;
        const MODE_OFF: u32 = 0x15;
        const ARG_SRC: u32 = 0x017976a4;
        const DEVICE: u32 = 0x01614c90;
        const HANDLE_OFF: u32 = 0x938;
        let idx = ((this + INDEX_OFF) as *const u32).read_unaligned();
        let k = idx.wrapping_mul(ROW_STRIDE);
        if ((relocated(ENABLE_ROW) + k) as *const u8).read() != 1 {
            return k;
        }
        let st = ((relocated(STATE_ROW) + k) as *const u32).read_unaligned();
        if st != 0 && st != 2 {
            return st;
        }
        let m = ((relocated(MODE_ROW) + k) as *const u32).read_unaligned();
        if m == 5 {
            ((this + MODE_OFF) as *mut u16).write_unaligned(0x0101);
        } else if m == 3 {
            ((this + MODE_OFF) as *mut u16).write_unaligned(1);
        } else if m == 4 {
            ((this + MODE_OFF) as *mut u16).write_unaligned(0x0100);
        }
        let b = global::<u8>(ARG_SRC).read();
        callee_cdecl!(DRAW, u32, this + 0x900, this + 0x8f8, this + 0x8fc, this, b as u32, 0);
        let h = callee_thiscall!(QUERY, u32, relocated(DEVICE));
        ((this + HANDLE_OFF) as *mut u32).write_unaligned(h);
        callee_cdecl!(COMMIT, u32, h, idx);
        callee_cdecl!(EPILOGUE, u32,)
    }
});
