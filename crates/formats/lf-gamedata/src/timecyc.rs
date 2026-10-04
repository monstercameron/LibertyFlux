//! Time cycle: `timecyc.dat` and `timecyclemodifiers*.dat`.
//!
//! Grammar: `//` comments, whitespace-separated rows.
//!
//! `timecyc.dat` holds one block per weather (`EXTRASUNNY`, `SUNNY`,
//! `SUNNY_WINDY`, `CLOUDY`, `RAIN`, `DRIZZLE`, `FOGGY`, `LIGHTNING`,
//! `TEMP`), each with 11 time slots (midnight, 5AM, 6AM, 7AM, 9AM,
//! midday, 6PM, 7PM, 8PM, 9PM, 10PM). Every slot row carries 134 values:
//! light and sky colours, sun and cloud parameters, water colour,
//! exposure and tone mapping, colour correction, depth of field and
//! post-effect tuning. The in-file header comment names the first 73;
//! the remaining 61 trailing floats have no documented meaning and are
//! kept as [`TimecycRow::extra`].
//!
//! `timecyclemodifiers*.dat` holds one row per named region/interior
//! modifier: a name plus 69 numbers (far-clip and fog ranges, ambient
//! and directional light overrides, fog colours, then undocumented
//! trailing values kept as [`Modifier::rest`]).

use crate::text::{CommentStyle, logical_lines, parse_f32, parse_u8, split_ws};
use crate::{Error, ErrorKind, Result, decode};

/// Colours are `[r, g, b]` 0-255; water adds alpha.
pub type Rgb = [u8; 3];
/// Water colour `[r, g, b, a]`.
pub type Rgba = [u8; 4];

/// One time slot row: 73 named values plus undocumented trailing floats.
#[derive(Debug, Clone)]
pub struct TimecycRow {
    /// Ambient light colour 0.
    pub amb0: Rgb,
    /// Ambient light colour 1.
    pub amb1: Rgb,
    /// Directional light colour.
    pub dir: Rgb,
    /// Sky top colour.
    pub sky_top: Rgb,
    /// Sky bottom colour.
    pub sky_bottom: Rgb,
    /// Sun core colour.
    pub sun_core: Rgb,
    /// Sun corona colour.
    pub sun_corona: Rgb,
    /// Sun size.
    pub sun_size: f32,
    /// Sprite brightness.
    pub sprite_brightness: f32,
    /// Far clip distance.
    pub far_clip: f32,
    /// Fog start.
    pub fog_start: f32,
    /// Low clouds colour.
    pub low_clouds: Rgb,
    /// Bottom clouds colour.
    pub bottom_clouds: Rgb,
    /// Water colour.
    pub water: Rgba,
    /// Exposure.
    pub exposure: f32,
    /// Bloom threshold.
    pub bloom_threshold: f32,
    /// Mid grey value.
    pub mid_grey: f32,
    /// Bloom intensity.
    pub bloom_intensity: f32,
    /// Colour correction RGB.
    pub colour_correct: Rgb,
    /// Colour add RGB.
    pub colour_add: Rgb,
    /// Desaturation.
    pub desaturation: f32,
    /// Contrast.
    pub contrast: f32,
    /// Gamma.
    pub gamma: f32,
    /// Far desaturation.
    pub desaturation_far: f32,
    /// Far contrast.
    pub contrast_far: f32,
    /// Far gamma.
    pub gamma_far: f32,
    /// Depth FX near.
    pub depth_fx_near: f32,
    /// Depth FX far.
    pub depth_fx_far: f32,
    /// Luminance min/max and adaptation delay.
    pub lum: [f32; 3],
    /// Cloud alpha.
    pub cloud_alpha: f32,
    /// Directional light multiplier.
    pub dir_light_mult: f32,
    /// Ambient light multipliers 0 and 1.
    pub amb_light_mult: [f32; 2],
    /// Sky light multiplier.
    pub sky_light_mult: f32,
    /// Sun multiplier.
    pub sun_mult: f32,
    /// Temperature.
    pub temperature: f32,
    /// Far/near depth of field.
    pub dof: [f32; 2],
    /// Near/mid/far DOF blur.
    pub dof_blur: [f32; 3],
    /// Water reflection strength.
    pub water_reflection: f32,
    /// Particle HDR factor.
    pub particle_hdr: f32,
    /// Sprite size.
    pub sprite_size: f32,
    /// Global sun multiplier.
    pub global_sun_mult: f32,
    /// Ambient occlusion scaler.
    pub ao_scaler: f32,
    /// Trailing undocumented values (61 per shipped row).
    pub extra: Vec<f32>,
}

/// One time slot: label plus row.
#[derive(Debug, Clone)]
pub struct TimeSlot {
    /// Slot label (`Midnight`, `5AM`, ..., `10PM`).
    pub label: String,
    /// Slot values.
    pub row: TimecycRow,
}

/// One weather block: name plus slots.
#[derive(Debug, Clone)]
pub struct Weather {
    /// Weather name (`EXTRASUNNY`, ...).
    pub name: String,
    /// Slots in file order.
    pub slots: Vec<TimeSlot>,
}

fn rgb(file: &str, num: usize, f: &[String], i: &mut usize) -> Result<Rgb> {
    let v = [
        parse_u8(file, num, f, *i)?,
        parse_u8(file, num, f, *i + 1)?,
        parse_u8(file, num, f, *i + 2)?,
    ];
    *i += 3;
    Ok(v)
}

fn fl(file: &str, num: usize, f: &[String], i: &mut usize) -> Result<f32> {
    let v = parse_f32(file, num, f, *i)?;
    *i += 1;
    Ok(v)
}

fn timecyc_row(file: &str, num: usize, f: &[String]) -> Result<TimecycRow> {
    // 73 named values minimum; shipped rows carry 134.
    if f.len() < 73 {
        return Err(Error::new(
            file,
            num,
            ErrorKind::FieldCount {
                expected: None,
                found: f.len(),
            },
        ));
    }
    let mut i = 0;
    let row = TimecycRow {
        amb0: rgb(file, num, f, &mut i)?,
        amb1: rgb(file, num, f, &mut i)?,
        dir: rgb(file, num, f, &mut i)?,
        sky_top: rgb(file, num, f, &mut i)?,
        sky_bottom: rgb(file, num, f, &mut i)?,
        sun_core: rgb(file, num, f, &mut i)?,
        sun_corona: rgb(file, num, f, &mut i)?,
        sun_size: fl(file, num, f, &mut i)?,
        sprite_brightness: fl(file, num, f, &mut i)?,
        far_clip: fl(file, num, f, &mut i)?,
        fog_start: fl(file, num, f, &mut i)?,
        low_clouds: rgb(file, num, f, &mut i)?,
        bottom_clouds: rgb(file, num, f, &mut i)?,
        water: {
            let w = [
                parse_u8(file, num, f, i)?,
                parse_u8(file, num, f, i + 1)?,
                parse_u8(file, num, f, i + 2)?,
                parse_u8(file, num, f, i + 3)?,
            ];
            i += 4;
            w
        },
        exposure: fl(file, num, f, &mut i)?,
        bloom_threshold: fl(file, num, f, &mut i)?,
        mid_grey: fl(file, num, f, &mut i)?,
        bloom_intensity: fl(file, num, f, &mut i)?,
        colour_correct: rgb(file, num, f, &mut i)?,
        colour_add: rgb(file, num, f, &mut i)?,
        desaturation: fl(file, num, f, &mut i)?,
        contrast: fl(file, num, f, &mut i)?,
        gamma: fl(file, num, f, &mut i)?,
        desaturation_far: fl(file, num, f, &mut i)?,
        contrast_far: fl(file, num, f, &mut i)?,
        gamma_far: fl(file, num, f, &mut i)?,
        depth_fx_near: fl(file, num, f, &mut i)?,
        depth_fx_far: fl(file, num, f, &mut i)?,
        lum: [
            fl(file, num, f, &mut i)?,
            fl(file, num, f, &mut i)?,
            fl(file, num, f, &mut i)?,
        ],
        cloud_alpha: fl(file, num, f, &mut i)?,
        dir_light_mult: fl(file, num, f, &mut i)?,
        amb_light_mult: [fl(file, num, f, &mut i)?, fl(file, num, f, &mut i)?],
        sky_light_mult: fl(file, num, f, &mut i)?,
        sun_mult: fl(file, num, f, &mut i)?,
        temperature: fl(file, num, f, &mut i)?,
        dof: [fl(file, num, f, &mut i)?, fl(file, num, f, &mut i)?],
        dof_blur: [
            fl(file, num, f, &mut i)?,
            fl(file, num, f, &mut i)?,
            fl(file, num, f, &mut i)?,
        ],
        water_reflection: fl(file, num, f, &mut i)?,
        particle_hdr: fl(file, num, f, &mut i)?,
        sprite_size: fl(file, num, f, &mut i)?,
        global_sun_mult: fl(file, num, f, &mut i)?,
        ao_scaler: fl(file, num, f, &mut i)?,
        extra: {
            let mut rest = Vec::with_capacity(f.len().saturating_sub(i));
            while i < f.len() {
                rest.push(parse_f32(file, num, f, i)?);
                i += 1;
            }
            rest
        },
    };
    debug_assert_eq!(i, f.len());
    Ok(row)
}

/// Parse a `timecyc.dat` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_timecyc(file: &str, bytes: &[u8]) -> Result<Vec<Weather>> {
    let text = decode(file, bytes)?;
    let mut out: Vec<Weather> = Vec::new();
    let mut pending_label: Option<String> = None;
    for l in logical_lines(&text, CommentStyle::SLASHES, true) {
        if l.code.is_empty() {
            if let Some(c) = &l.comment {
                // Weather headers look like `////////// EXTRASUNNY`: the
                // comment text after stripping `//` is `//////// EXTRASUNNY`.
                let name = c.trim_matches('/').trim();
                if l.comment
                    .as_deref()
                    .is_some_and(|full| full.starts_with("///////"))
                    && !name.is_empty()
                    && !name.contains(' ')
                {
                    out.push(Weather {
                        name: name.to_string(),
                        slots: Vec::new(),
                    });
                    pending_label = None;
                    continue;
                }
                // Slot labels (`//Midnight`) and the column header comment.
                if !c.contains(' ') && !c.is_empty() && out.last().is_some() {
                    pending_label = Some(c.clone());
                }
            }
            continue;
        }
        let w = out.last_mut().ok_or_else(|| {
            Error::new(
                file,
                l.num,
                ErrorKind::UnknownMarker {
                    marker: "row before first weather".to_string(),
                },
            )
        })?;
        let f = split_ws(&l.code);
        w.slots.push(TimeSlot {
            label: pending_label.take().unwrap_or_default(),
            row: timecyc_row(file, l.num, &f)?,
        });
    }
    Ok(out)
}

/// One time-cycle modifier row: name plus values.
#[derive(Debug, Clone)]
pub struct Modifier {
    /// Modifier name.
    pub name: String,
    /// Min/max far clip.
    pub far_clip: [f32; 2],
    /// Min/max fog start.
    pub fog_start: [f32; 2],
    /// Ambient light 0: RGBA plus multiplier.
    pub ambient0: [f32; 5],
    /// Ambient light 1: RGBA plus multiplier.
    pub ambient1: [f32; 5],
    /// Directional light: RGBA plus multiplier.
    pub directional: [f32; 5],
    /// Ambient down multiplier.
    pub ambient_down_mult: f32,
    /// Fog colour RGBA.
    pub fog: [f32; 4],
    /// Near fog colour RGBA.
    pub near_fog: [f32; 4],
    /// Remaining undocumented values.
    pub rest: Vec<f32>,
}

/// Parse a `timecyclemodifiers*.dat` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_modifiers(file: &str, bytes: &[u8]) -> Result<Vec<Modifier>> {
    let text = decode(file, bytes)?;
    let mut out = Vec::new();
    for l in logical_lines(&text, CommentStyle::SLASHES, false) {
        let f = split_ws(&l.code);
        // Name + 28 documented numbers minimum; shipped rows carry 70.
        if f.len() < 29 {
            return Err(Error::new(
                file,
                l.num,
                ErrorKind::FieldCount {
                    expected: None,
                    found: f.len(),
                },
            ));
        }
        let n = l.num;
        let fl = |i: usize| parse_f32(file, n, &f, i);
        let mut i = 1;
        let take = |count: usize, i: &mut usize| -> Result<Vec<f32>> {
            let mut v = Vec::with_capacity(count);
            for _ in 0..count {
                v.push(fl(*i)?);
                *i += 1;
            }
            Ok(v)
        };
        let far_clip = take(2, &mut i)?;
        let fog_start = take(2, &mut i)?;
        let ambient0 = take(5, &mut i)?;
        let ambient1 = take(5, &mut i)?;
        let directional = take(5, &mut i)?;
        let ambient_down_mult = fl(i)?;
        i += 1;
        let fog = take(4, &mut i)?;
        let near_fog = take(4, &mut i)?;
        let mut rest = Vec::new();
        while i < f.len() {
            rest.push(fl(i)?);
            i += 1;
        }
        out.push(Modifier {
            name: f[0].clone(),
            far_clip: [far_clip[0], far_clip[1]],
            fog_start: [fog_start[0], fog_start[1]],
            ambient0: ambient0.try_into().unwrap_or([0.0; 5]),
            ambient1: ambient1.try_into().unwrap_or([0.0; 5]),
            directional: directional.try_into().unwrap_or([0.0; 5]),
            ambient_down_mult,
            fog: fog.try_into().unwrap_or([0.0; 4]),
            near_fog: near_fog.try_into().unwrap_or([0.0; 4]),
            rest,
        });
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // parsed values are compared bit for bit on purpose
mod tests {
    use super::*;

    fn row73() -> String {
        // 73 named values: colours as ints, rest as floats.
        let mut v: Vec<&str> = Vec::new();
        for _ in 0..7 {
            v.extend(["1", "2", "3"]);
        }
        v.extend(["1.0", "1.0", "1500.0", "50.0"]); // sun..fog
        v.extend(["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]); // clouds+water
        v.extend(["1.0"; 4]);
        v.extend(["1", "2", "3", "4", "5", "6"]); // cc+ca
        v.extend(["1.0"; 28]);
        assert_eq!(v.len(), 73);
        v.join(" ")
    }

    #[test]
    fn parses_weather_and_slot() {
        let src = format!("////////// EXTRASUNNY\r\n//Midnight\r\n{}\r\n", row73());
        let w = parse_timecyc("timecyc.dat", src.as_bytes()).unwrap();
        assert_eq!(w.len(), 1);
        assert_eq!(w[0].name, "EXTRASUNNY");
        assert_eq!(w[0].slots.len(), 1);
        assert_eq!(w[0].slots[0].label, "Midnight");
        assert_eq!(w[0].slots[0].row.amb0, [1, 2, 3]);
        assert_eq!(w[0].slots[0].row.far_clip, 1500.0);
    }

    #[test]
    fn parses_modifier() {
        let mut v = vec!["noambient".to_string()];
        for _ in 0..69 {
            v.push("1.0".to_string());
        }
        let src = format!("// header comment\r\n{}\r\n", v.join(" "));
        let m = parse_modifiers("timecyclemodifiers.dat", src.as_bytes()).unwrap();
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].name, "noambient");
        assert_eq!(m[0].rest.len(), 69 - 28);
    }
}
