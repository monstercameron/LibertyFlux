// original: 0x00A2E740 weather_apply_or_release

/// Applies the requested weather transition and refreshes the cached
/// time-of-day scalar.
///
/// The cdecl entry takes three 32-bit words: a weather selector, an index
/// into the forecast-byte table, and a low-byte transition flag. With a
/// nonzero flag it calls the immediate-apply routine with the selector. With
/// a zero flag it calls the weather update routine, conditionally marks the
/// time-of-day object dirty only when both selectors differ from their cached
/// value, stores both selector words, and reads a signed forecast byte at the
/// requested index. Both paths then compute
/// `(selected_minute + selected_hour * scale) * scale`, where the two integer inputs
/// are converted as signed 32-bit values and `scale` is read from the image.
/// The function writes the selected forecast byte and the exact `f32` bits to
/// separate globals; its return is the immediate-apply answer or the signed
/// forecast byte, depending on the path.
///
/// Calls to the three engine helpers are intercepted by the proof contract.
/// The time-of-day object pointer is read from its relocated global and its
/// dirty word is written at byte offset `0x260` only on a changed selector.
lf_checker_rt::export!(cdecl, rw_00a2e740(weather_selector: u32, forecast_index: u32, apply_immediately: u32) -> u32 {
    const APPLY_WEATHER: u32 = 1;
    const UPDATE_WEATHER: u32 = 2;
    const SELECT_FORECAST_TABLE: u32 = 3;

    const WEATHER_SELECTOR_CACHE: u32 = 0x012D_DE80;
    const FORECAST_INDEX_CACHE: u32 = 0x012D_DE90;
    const FORECAST_VALUE_CACHE: u32 = 0x012D_DE84;
    const TIME_SCALAR_CACHE: u32 = 0x012D_DE94;
    const TIME_OF_DAY_OBJECT: u32 = 0x011A_28F4;
    const SELECTED_HOUR: u32 = 0x0129_5850;
    const SELECTED_MINUTE: u32 = 0x0129_584C;
    const OPTIONAL_MINUTE: u32 = 0x0129_5858;
    const MINUS_ONE: i32 = -1;
    const TIME_SCALE: u32 = 0x00FE_8724;
    const DIRTY_WORD_OFFSET: usize = 0x260 / 4;

    unsafe {
        let returned = if apply_immediately as u8 != 0 {
            lf_checker_rt::callee_cdecl!(APPLY_WEATHER, u32, weather_selector)
        } else {
            let _update_answer = lf_checker_rt::callee_cdecl!(UPDATE_WEATHER, u32,);

            let previous_weather =
                lf_checker_rt::global::<u32>(WEATHER_SELECTOR_CACHE).read_unaligned();
            let previous_index =
                lf_checker_rt::global::<u32>(FORECAST_INDEX_CACHE).read_unaligned();

            if previous_weather != weather_selector && previous_index != forecast_index {
                let object = lf_checker_rt::global::<u32>(TIME_OF_DAY_OBJECT).read_unaligned();
                (object as *mut u32)
                    .add(DIRTY_WORD_OFFSET)
                    .write_unaligned(2);
            }

            lf_checker_rt::global::<u32>(FORECAST_INDEX_CACHE)
                .write_unaligned(forecast_index);
            lf_checker_rt::global::<u32>(WEATHER_SELECTOR_CACHE)
                .write_unaligned(weather_selector);

            let table = lf_checker_rt::callee_cdecl!(SELECT_FORECAST_TABLE, u32,);
            let value = (table as *const u8)
                .wrapping_add(forecast_index as usize)
                .read();
            let selected = i32::from(value as i8);
            lf_checker_rt::global::<i32>(FORECAST_VALUE_CACHE).write_unaligned(selected);
            selected as u32
        };

        let scale = lf_checker_rt::global::<f32>(TIME_SCALE).read_unaligned();
        let hour = lf_checker_rt::global::<i32>(SELECTED_HOUR).read_unaligned() as f32;
        let minute_word = lf_checker_rt::global::<i32>(OPTIONAL_MINUTE).read_unaligned();
        let minute = if minute_word != MINUS_ONE {
            minute_word
        } else {
            lf_checker_rt::global::<i32>(SELECTED_MINUTE).read_unaligned()
        } as f32;

        let scaled_hour =
            core::hint::black_box(hour) * core::hint::black_box(scale);
        let total = core::hint::black_box(minute) + core::hint::black_box(scaled_hour);
        let time_scalar = core::hint::black_box(total) * core::hint::black_box(scale);
        lf_checker_rt::global::<u32>(TIME_SCALAR_CACHE)
            .write_unaligned(time_scalar.to_bits());

        returned
    }
});
