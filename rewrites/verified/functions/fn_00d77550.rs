// original: 0x00D77550 CRenderPhaseWaterReflection::vf3

/// Render the water-reflection phase unless a gate skips it.
///
/// Returns early when the enable flag is 1, or when the context pointer
/// is non-null with bit 0 of its byte at `+0x22` set (returning the
/// pointer's upper bytes with bit 0 set, or 0 when the flag gate hit with
/// incoming `eax` fixed to 0 by the contract). Otherwise queries the
/// device, stores the handle at `+0x938`, issues a six-argument draw over
/// the blocks at `+0x8fc`/`+0x8f8`/`+0x900`, commits the handle, and
/// tail-jumps to the epilogue, whose answer is returned. Thiscall:
/// object in `ecx`, no stack words.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const QUERY: u32 = 1;
const DRAW: u32 = 2;
const COMMIT: u32 = 3;
const EPILOGUE: u32 = 4;

export!(thiscall, rw_00d77550(this: u32) -> u32 {
    unsafe {
        const ENABLED: u32 = 0x0166da20;
        const CONTEXT: u32 = 0x016dd67c;
        const CTX_FLAG_OFF: u32 = 0x22;
        const DEVICE: u32 = 0x01614c90;
        const HANDLE_OFF: u32 = 0x938;
        if global::<u8>(ENABLED).read() == 1 {
            return 0;
        }
        let ctx = global::<u32>(CONTEXT).read();
        if ctx != 0 && ((ctx + CTX_FLAG_OFF) as *const u8).read() & 1 != 0 {
            return (ctx & 0xffff_ff00) | 1;
        }
        let h = callee_thiscall!(QUERY, u32, relocated(DEVICE));
        ((this + HANDLE_OFF) as *mut u32).write_unaligned(h);
        callee_cdecl!(DRAW, u32, this + 0x900, this + 0x8f8, this + 0x8fc, this, 1, 0);
        let h = ((this + HANDLE_OFF) as *const u32).read_unaligned();
        callee_cdecl!(COMMIT, u32, h);
        callee_cdecl!(EPILOGUE, u32,)
    }
});
