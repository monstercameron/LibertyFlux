// original: 0x00963d90 renderer_open
/// Open the renderer unless its generation is already current.
///
/// Takes one handle word. Returns at once (generation with low byte
/// cleared) unless the open generation at `0x12088B4` already equals the
/// wanted constant `0xFFFFFFFF` at `0xF1C040`. On a match it raises the
/// opening flags, runs the 7-argument create helper (whose answer becomes
/// the new generation) and returns again when that still equals the wanted
/// generation. On a fresh generation it records the
/// device word from the 1-argument helper at `0x11F703C`, initialises the
/// two engine objects (thiscall, fixed object pointers), runs the extra
/// initialiser when the second reports a non-zero low byte, submits the
/// mode byte from `0x18B6E8D` to the third object, and runs the two finish
/// helpers. The early exits return the compared generation, or the
/// create answer, with the low byte cleared; the end returns the last
/// finish answer with the low byte forced to 1.
lf_checker_rt::export!(cdecl, rw_00963d90(handle: u32) -> u32 {
    unsafe {
        const WANT: u32 = 0xf1c040;
        const OPEN: u32 = 0x12088b4;
        const OPENING: u32 = 0x11f7039;
        const READY: u32 = 0x10376e8;
        const STATE: u32 = 0x10376ec;
        const DEVICE: u32 = 0x11f703c;
        const MODEBYTE: u32 = 0x18b6e8d;
        const HI: u32 = 0xffff_ff00;
        let g = |va: u32| (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned();
        if g(OPEN) != g(WANT) {
            return g(OPEN) & HI;
        }
        (lf_checker_rt::global::<u8>(OPENING) as *mut u8).write(0);
        (lf_checker_rt::global::<u8>(READY) as *mut u8).write(1);
        (lf_checker_rt::global::<u32>(STATE) as *mut u32).write_unaligned(0);
        // Code-pointer constants are relocated immediates in the original.
        let gen: u32 = lf_checker_rt::callee_cdecl!(
            1, u32, lf_checker_rt::relocated(0x955830), handle, 0x6000, 0,
            lf_checker_rt::relocated(0xe8acb8), 1, 0);
        (lf_checker_rt::global::<u32>(OPEN) as *mut u32).write_unaligned(gen);
        if gen == g(WANT) {
            return gen & HI;
        }
        let dev: u32 = lf_checker_rt::callee_cdecl!(2, u32, 0);
        (lf_checker_rt::global::<u32>(DEVICE) as *mut u32).write_unaligned(dev);
        // Pushed 1, 0, 0, so the stack arguments read (0, 0, 1).
        let _: u32 = lf_checker_rt::callee_thiscall!(
            3, u32, lf_checker_rt::relocated(0x128e310), 0, 0, 1);
        let rep: u32 =
            lf_checker_rt::callee_thiscall!(4, u32, lf_checker_rt::relocated(0x1161518),);
        if (rep & 0xff) != 0 {
            let _: u32 =
                lf_checker_rt::callee_thiscall!(5, u32, lf_checker_rt::relocated(0x11737d0),);
        }
        let mode = (lf_checker_rt::global::<u8>(MODEBYTE) as *const u8).read() as u32;
        let _: u32 =
            lf_checker_rt::callee_thiscall!(6, u32, lf_checker_rt::relocated(0x11737d0), mode);
        let _: u32 = lf_checker_rt::callee_cdecl!(7, u32,);
        let fin: u32 = lf_checker_rt::callee_cdecl!(8, u32,);
        (fin & HI) | 1
    }
});
