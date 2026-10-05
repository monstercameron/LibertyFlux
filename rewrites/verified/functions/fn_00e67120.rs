// original: 0x00E67120 task_float_fanout_00E67120
/// Fan out two task floats to twelve globals.
///
/// Loads `FIRST` and `SECOND` (global single-precision floats) once,
/// then stores them alternately to the twelve destination globals
/// (`DST0` to `DST11`, even slots taking `FIRST`), as plain bit moves.
///
/// Original: 0x00E67120 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E67120() -> u32 {
    unsafe {
        const FIRST: u32 = 0x01050B4C;
        const SECOND: u32 = 0x01050B50;
        const DST0: u32 = 0x0103C134;
        const DST1: u32 = 0x0103C138;
        const DST2: u32 = 0x0103C15C;
        const DST3: u32 = 0x0103C160;
        const DST4: u32 = 0x0103C184;
        const DST5: u32 = 0x0103C188;
        const DST6: u32 = 0x0103C314;
        const DST7: u32 = 0x0103C318;
        const DST8: u32 = 0x0103C33C;
        const DST9: u32 = 0x0103C340;
        const DST10: u32 = 0x0103C364;
        const DST11: u32 = 0x0103C368;
        let first: u32 = (lf_checker_rt::global::<u32>(FIRST)).read_unaligned();
        let second: u32 = (lf_checker_rt::global::<u32>(SECOND)).read_unaligned();
        (lf_checker_rt::global::<u32>(DST0)).write_unaligned(first);
        (lf_checker_rt::global::<u32>(DST1)).write_unaligned(second);
        (lf_checker_rt::global::<u32>(DST2)).write_unaligned(first);
        (lf_checker_rt::global::<u32>(DST3)).write_unaligned(second);
        (lf_checker_rt::global::<u32>(DST4)).write_unaligned(first);
        (lf_checker_rt::global::<u32>(DST5)).write_unaligned(second);
        (lf_checker_rt::global::<u32>(DST6)).write_unaligned(first);
        (lf_checker_rt::global::<u32>(DST7)).write_unaligned(second);
        (lf_checker_rt::global::<u32>(DST8)).write_unaligned(first);
        (lf_checker_rt::global::<u32>(DST9)).write_unaligned(second);
        (lf_checker_rt::global::<u32>(DST10)).write_unaligned(first);
        (lf_checker_rt::global::<u32>(DST11)).write_unaligned(second);
        0
    }
});
