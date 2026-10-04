//! `handling.dat`: vehicle physics tuning.
//!
//! One row per handling id, whitespace separated, `;` comments. After the
//! car rows come four sigil-led sub-tables: `%` boats, `!` bikes, `$`
//! flying (helicopters and leftover planes) and `^` vehicle anim groups.
//! A `#` starts a disabled row (used to comment out one flying row).
//!
//! Column meanings come from the header comments in the shipped file
//! itself (units, ranges, flag bits). Two spots need care:
//!
//! * The description block lists six traction fields but the column ruler
//!   and every data row carry five; the longitudinal curve field from the
//!   older layout is gone (inferred: the ruler matches the data).
//! * The column ruler shows a handbrake field (`Thb`) between brake bias
//!   and steering lock that the description block never names; it is
//!   parsed as handbrake force (inferred from position and values).
//!
//! Sub-table label rows are two wrapped comment lines; the pairing of
//! labels to columns used here groups each pair/triple naturally and is
//! marked inferred in `format-spec.md`.

use crate::text::{
    CommentStyle, field, logical_lines, parse_f32, parse_hex_u32, parse_i32, split_ws,
};
use crate::{Error, ErrorKind, Result, decode};

/// Full contents of a `handling.dat` file.
#[derive(Debug, Clone, Default)]
pub struct HandlingData {
    /// Car/truck handling rows (37 whitespace fields each).
    pub cars: Vec<CarHandling>,
    /// Boat rows (`%`, marker + name + 19 values).
    pub boats: Vec<BoatHandling>,
    /// Bike rows (`!`, marker + name + 15 values).
    pub bikes: Vec<BikeHandling>,
    /// Flying rows (`$`, marker + name + 22 values).
    pub flying: Vec<FlyingHandling>,
    /// Vehicle anim group rows (`^`, marker + id + 6 names + 15 values).
    pub anim_groups: Vec<AnimGroup>,
}

/// One car handling row. Units: mass kg, velocity km/h, angles degrees.
#[derive(Debug, Clone)]
pub struct CarHandling {
    /// Handling id, max 14 chars (e.g. `ADMIRAL`).
    pub name: String,
    /// (B) mass in kg.
    pub mass: f32,
    /// (C) drag multiplier.
    pub drag_mult: f32,
    /// (D) percent submerged, 10-120.
    pub percent_submerged: i32,
    /// (E-G) centre of mass in metres.
    pub centre_of_mass: [f32; 3],
    /// (Tt) drive bias: 1.0 = front wheel drive, 0.0 = rear.
    pub drive_bias: f32,
    /// (Tg) drive gear count.
    pub drive_gears: i32,
    /// (Tf) drive force.
    pub drive_force: f32,
    /// (Ti) drive inertia.
    pub drive_inertia: f32,
    /// (Tv) velocity (top speed figure).
    pub velocity: f32,
    /// (Tb) brake force.
    pub brake_force: f32,
    /// (Tbb) brake bias.
    pub brake_bias: f32,
    /// (Thb) handbrake force (inferred: in ruler, missing from descriptions).
    pub handbrake: f32,
    /// (Ts) steering lock in degrees.
    pub steering_lock: f32,
    /// (Wc+) traction curve maximum.
    pub traction_max: f32,
    /// (Wc-) traction curve minimum.
    pub traction_min: f32,
    /// (Wc-) lateral traction curve shape (peak position, degrees).
    pub traction_lateral: f32,
    /// (Ws+) traction spring delta max.
    pub traction_spring_delta_max: f32,
    /// (Wh) traction bias.
    pub traction_bias: f32,
    /// (Sf) suspension force.
    pub suspension_force: f32,
    /// (Scd) suspension compression damping.
    pub suspension_comp_damp: f32,
    /// (Srd) suspension rebound damping.
    pub suspension_rebound_damp: f32,
    /// (Su) suspension upper limit.
    pub suspension_upper_limit: f32,
    /// (Sl) suspension lower limit.
    pub suspension_lower_limit: f32,
    /// (Sr) suspension raise.
    pub suspension_raise: f32,
    /// (Sb) suspension bias.
    pub suspension_bias: f32,
    /// (Dc) collision damage multiplier.
    pub collision_damage_mult: f32,
    /// (Dw) weapon damage multiplier.
    pub weapon_damage_mult: f32,
    /// (Dd) deformation damage multiplier.
    pub deformation_damage_mult: f32,
    /// (De) engine damage multiplier.
    pub engine_damage_mult: f32,
    /// (Ms) seat offset distance.
    pub seat_offset_dist: f32,
    /// (Mv) monetary value.
    pub monetary_value: i32,
    /// (Mmf) model flags, written as hex (`IS_VAN`, `IS_BUS`, ...).
    pub model_flags: u32,
    /// (Mhf) handling flags, written as hex.
    pub handling_flags: u32,
    /// (Ma) anim group id, indexes the `^` table.
    pub anim_group: i32,
}

/// One boat handling row (`%`). Some car fields are reused for boats.
#[derive(Debug, Clone)]
pub struct BoatHandling {
    /// Handling id.
    pub name: String,
    /// Bounding box forward extent.
    pub bbox_fwd: f32,
    /// Bounding box side extent.
    pub bbox_side: f32,
    /// Bounding box back extent.
    pub bbox_back: f32,
    /// Sample bottom.
    pub sample_bottom: f32,
    /// Sample top.
    pub sample_top: f32,
    /// Aquaplane force.
    pub aquaplane_force: f32,
    /// Aquaplane wave multiplier.
    pub aquaplane_wave_mult: f32,
    /// Aquaplane wave cap.
    pub aquaplane_wave_cap: f32,
    /// Aquaplane wave app.
    pub aquaplane_wave_app: f32,
    /// Rudder force.
    pub rudder_f: f32,
    /// Rudder offset.
    pub rudder_offset: f32,
    /// Wave audio multiplier.
    pub wave_audio_mult: f32,
    /// Move resistance XY.
    pub move_res_xy: f32,
    /// Move resistance Z up.
    pub move_res_z_up: f32,
    /// Move resistance Z down.
    pub move_res_z_down: f32,
    /// Turn resistance X.
    pub turn_res_x: f32,
    /// Turn resistance Y.
    pub turn_res_y: f32,
    /// Turn resistance Z.
    pub turn_res_z: f32,
    /// Look left/right behind camera height.
    pub look_lr_behind_cam_height: f32,
}

/// One bike handling row (`!`).
#[derive(Debug, Clone)]
pub struct BikeHandling {
    /// Handling id.
    pub name: String,
    /// Lean forward centre of mass.
    pub lean_fwd_com: f32,
    /// Lean forward force.
    pub lean_fwd_force: f32,
    /// Lean back centre of mass.
    pub lean_back_com: f32,
    /// Lean back force.
    pub lean_back_force: f32,
    /// Maximum lean.
    pub max_lean: f32,
    /// Full anim lean.
    pub full_anim_lean: f32,
    /// Desired lean.
    pub desired_lean: f32,
    /// Stick lean.
    pub stick_lean: f32,
    /// Brake stabilisation.
    pub brake_stabil: f32,
    /// In-air steering.
    pub in_air_steer: f32,
    /// Wheelie angle.
    pub wheelie_angle: f32,
    /// Stoppie angle.
    pub stoppie_angle: f32,
    /// Wheelie steering.
    pub wheelie_steer: f32,
    /// Wheelie stability multiplier.
    pub wheelie_stab_mult: f32,
    /// Stoppie stability multiplier.
    pub stoppie_stab_mult: f32,
}

/// One flying handling row (`$`).
#[derive(Debug, Clone)]
pub struct FlyingHandling {
    /// Handling id.
    pub name: String,
    /// Thrust.
    pub thrust: f32,
    /// Thrust falloff.
    pub thrust_falloff: f32,
    /// Thrust vector.
    pub thrust_vec: f32,
    /// Yaw.
    pub yaw: f32,
    /// Yaw stability.
    pub yaw_stab: f32,
    /// Side slip.
    pub side_slip: f32,
    /// Roll.
    pub roll: f32,
    /// Roll stability.
    pub roll_stab: f32,
    /// Pitch.
    pub pitch: f32,
    /// Pitch stability.
    pub pitch_stab: f32,
    /// Form lift.
    pub form_lift: f32,
    /// Attack lift.
    pub attack_lift: f32,
    /// Gear up (right?).
    pub gear_up: f32,
    /// Gear down (left?).
    pub gear_down: f32,
    /// Wind multiplier.
    pub wind_mult: f32,
    /// Move resistance.
    pub move_res: f32,
    /// Turn resistance.
    pub turn_res: [f32; 3],
    /// Speed resistance.
    pub speed_res: [f32; 3],
}

/// One vehicle anim group row (`^`).
#[derive(Debug, Clone)]
pub struct AnimGroup {
    /// Group id, referenced by car rows.
    pub id: i32,
    /// Enter anim folder, and its fallback.
    pub enter: [String; 2],
    /// Jacking anim folder, and its fallback.
    pub jack: [String; 2],
    /// Driving anim folder, and its fallback.
    pub drive: [String; 2],
    /// Time to get in.
    pub get_in_time: f32,
    /// Time to get out.
    pub get_out_time: f32,
    /// Jump-out time.
    pub jump_out_time: f32,
    /// Jacked-out time.
    pub jacked_out_time: f32,
    /// Fall time.
    pub fall_time: f32,
    /// Door open-out start/stop.
    pub open_out: [f32; 2],
    /// Door close-in start/stop.
    pub close_in: [f32; 2],
    /// Door open-in start/stop.
    pub open_in: [f32; 2],
    /// Door close-out start/stop.
    pub close_out: [f32; 2],
    /// Special flags bitmask (1/2/4/8/16/32/64, see file header).
    pub special_flag: i32,
}

fn car_row(file: &str, num: usize, f: &[String]) -> Result<CarHandling> {
    if f.len() != 37 {
        return Err(Error::new(
            file,
            num,
            ErrorKind::FieldCount {
                expected: Some(37),
                found: f.len(),
            },
        ));
    }
    let g = |i: usize| field(file, num, f, i, Some(37)).map(std::string::ToString::to_string);
    let fl = |i: usize| parse_f32(file, num, f, i);
    let ii = |i: usize| parse_i32(file, num, f, i);
    Ok(CarHandling {
        name: g(0)?,
        mass: fl(1)?,
        drag_mult: fl(2)?,
        percent_submerged: ii(3)?,
        centre_of_mass: [fl(4)?, fl(5)?, fl(6)?],
        drive_bias: fl(7)?,
        drive_gears: ii(8)?,
        drive_force: fl(9)?,
        drive_inertia: fl(10)?,
        velocity: fl(11)?,
        brake_force: fl(12)?,
        brake_bias: fl(13)?,
        handbrake: fl(14)?,
        steering_lock: fl(15)?,
        traction_max: fl(16)?,
        traction_min: fl(17)?,
        traction_lateral: fl(18)?,
        traction_spring_delta_max: fl(19)?,
        traction_bias: fl(20)?,
        suspension_force: fl(21)?,
        suspension_comp_damp: fl(22)?,
        suspension_rebound_damp: fl(23)?,
        suspension_upper_limit: fl(24)?,
        suspension_lower_limit: fl(25)?,
        suspension_raise: fl(26)?,
        suspension_bias: fl(27)?,
        collision_damage_mult: fl(28)?,
        weapon_damage_mult: fl(29)?,
        deformation_damage_mult: fl(30)?,
        engine_damage_mult: fl(31)?,
        seat_offset_dist: fl(32)?,
        monetary_value: ii(33)?,
        model_flags: parse_hex_u32(file, num, f, 34)?,
        handling_flags: parse_hex_u32(file, num, f, 35)?,
        anim_group: ii(36)?,
    })
}

fn boat_row(file: &str, num: usize, f: &[String]) -> Result<BoatHandling> {
    if f.len() != 21 {
        return Err(Error::new(
            file,
            num,
            ErrorKind::FieldCount {
                expected: Some(21),
                found: f.len(),
            },
        ));
    }
    let fl = |i: usize| parse_f32(file, num, f, i);
    Ok(BoatHandling {
        name: field(file, num, f, 1, Some(21))?.to_string(),
        bbox_fwd: fl(2)?,
        bbox_side: fl(3)?,
        bbox_back: fl(4)?,
        sample_bottom: fl(5)?,
        sample_top: fl(6)?,
        aquaplane_force: fl(7)?,
        aquaplane_wave_mult: fl(8)?,
        aquaplane_wave_cap: fl(9)?,
        aquaplane_wave_app: fl(10)?,
        rudder_f: fl(11)?,
        rudder_offset: fl(12)?,
        wave_audio_mult: fl(13)?,
        move_res_xy: fl(14)?,
        move_res_z_up: fl(15)?,
        move_res_z_down: fl(16)?,
        turn_res_x: fl(17)?,
        turn_res_y: fl(18)?,
        turn_res_z: fl(19)?,
        look_lr_behind_cam_height: fl(20)?,
    })
}

fn bike_row(file: &str, num: usize, f: &[String]) -> Result<BikeHandling> {
    if f.len() != 17 {
        return Err(Error::new(
            file,
            num,
            ErrorKind::FieldCount {
                expected: Some(17),
                found: f.len(),
            },
        ));
    }
    let fl = |i: usize| parse_f32(file, num, f, i);
    Ok(BikeHandling {
        name: field(file, num, f, 1, Some(17))?.to_string(),
        lean_fwd_com: fl(2)?,
        lean_fwd_force: fl(3)?,
        lean_back_com: fl(4)?,
        lean_back_force: fl(5)?,
        max_lean: fl(6)?,
        full_anim_lean: fl(7)?,
        desired_lean: fl(8)?,
        stick_lean: fl(9)?,
        brake_stabil: fl(10)?,
        in_air_steer: fl(11)?,
        wheelie_angle: fl(12)?,
        stoppie_angle: fl(13)?,
        wheelie_steer: fl(14)?,
        wheelie_stab_mult: fl(15)?,
        stoppie_stab_mult: fl(16)?,
    })
}

fn flying_row(file: &str, num: usize, f: &[String]) -> Result<FlyingHandling> {
    if f.len() != 24 {
        return Err(Error::new(
            file,
            num,
            ErrorKind::FieldCount {
                expected: Some(24),
                found: f.len(),
            },
        ));
    }
    let fl = |i: usize| parse_f32(file, num, f, i);
    Ok(FlyingHandling {
        name: field(file, num, f, 1, Some(24))?.to_string(),
        thrust: fl(2)?,
        thrust_falloff: fl(3)?,
        thrust_vec: fl(4)?,
        yaw: fl(5)?,
        yaw_stab: fl(6)?,
        side_slip: fl(7)?,
        roll: fl(8)?,
        roll_stab: fl(9)?,
        pitch: fl(10)?,
        pitch_stab: fl(11)?,
        form_lift: fl(12)?,
        attack_lift: fl(13)?,
        gear_up: fl(14)?,
        gear_down: fl(15)?,
        wind_mult: fl(16)?,
        move_res: fl(17)?,
        turn_res: [fl(18)?, fl(19)?, fl(20)?],
        speed_res: [fl(21)?, fl(22)?, fl(23)?],
    })
}

fn anim_row(file: &str, num: usize, f: &[String]) -> Result<AnimGroup> {
    if f.len() != 22 {
        return Err(Error::new(
            file,
            num,
            ErrorKind::FieldCount {
                expected: Some(22),
                found: f.len(),
            },
        ));
    }
    let g = |i: usize| field(file, num, f, i, Some(22)).map(std::string::ToString::to_string);
    let fl = |i: usize| parse_f32(file, num, f, i);
    Ok(AnimGroup {
        id: parse_i32(file, num, f, 1)?,
        enter: [g(2)?, g(3)?],
        jack: [g(4)?, g(5)?],
        drive: [g(6)?, g(7)?],
        get_in_time: fl(8)?,
        get_out_time: fl(9)?,
        jump_out_time: fl(10)?,
        jacked_out_time: fl(11)?,
        fall_time: fl(12)?,
        open_out: [fl(13)?, fl(14)?],
        close_in: [fl(15)?, fl(16)?],
        open_in: [fl(17)?, fl(18)?],
        close_out: [fl(19)?, fl(20)?],
        special_flag: parse_i32(file, num, f, 21)?,
    })
}

/// Parse a `handling.dat` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_handling(file: &str, bytes: &[u8]) -> Result<HandlingData> {
    let text = decode(file, bytes)?;
    let mut out = HandlingData::default();
    for l in logical_lines(&text, CommentStyle::SEMICOLON_HASH, false) {
        let f = split_ws(&l.code);
        if f.is_empty() {
            continue;
        }
        match f[0].as_str() {
            "%" => out.boats.push(boat_row(file, l.num, &f)?),
            "!" => out.bikes.push(bike_row(file, l.num, &f)?),
            "$" => out.flying.push(flying_row(file, l.num, &f)?),
            "^" => out.anim_groups.push(anim_row(file, l.num, &f)?),
            _ => out.cars.push(car_row(file, l.num, &f)?),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "; comment\r\nADMIRAL 1700.0 6.0 85 0.0 0.0 -0.2 0.0 5 0.17 1.0 140.0 0.22 0.65 0.7 35.0 1.2 0.95 14.0 0.13 0.47 1.6 1.0 1.0 0.15 -0.16 0.0 0.5 1.0 1.0 0.7 1.5 0.0 25000 440080 0 0\r\n% DINGHY 0.8 0.8 0.7 0.2 0.1 1.40 0.4 3.5 0.002 0.5 0.0 4.0 0.01 0.50 0.10 0.04 0.30 0.04 4.0\r\n! BOBBER 0.15 2.0 0.25 3.5 30.0 38.0 0.30 0.20 -0.5 -2.5 35.0 -40.0 -10.0 10.0 5.0\r\n$ ANNHIL 0.60 0.03 0.8 -1.10 0.001 0.005 1.40 0.010 1.50 0.0008 0.6 2.0 0.2 1.0 0.0 0.01 0.55 0.55 0.7 0.5 0.5 0.7\r\n^ 0 std std std std std std 0.5 0.0 -0.5 -0.3 0.3 0.41 0.8 0.3 0.45 0.06 0.43 0.20 0.43 0\r\n#$ FIGHTER 0.5 2.0 0.0\r\n";

    #[test]
    fn parses_all_subtables() {
        let h = parse_handling("handling.dat", SAMPLE.as_bytes()).unwrap();
        assert_eq!(h.cars.len(), 1);
        assert_eq!(h.boats.len(), 1);
        assert_eq!(h.bikes.len(), 1);
        assert_eq!(h.flying.len(), 1);
        assert_eq!(h.anim_groups.len(), 1);
        assert_eq!(h.cars[0].name, "ADMIRAL");
        assert_eq!(h.cars[0].model_flags, 0x440080);
        assert_eq!(h.cars[0].drive_gears, 5);
        assert_eq!(h.anim_groups[0].id, 0);
    }

    #[test]
    fn bad_field_count_errors() {
        let bad = "SHORT 1.0 2.0\r\n";
        let e = parse_handling("handling.dat", bad.as_bytes()).unwrap_err();
        assert_eq!(e.line, 1);
        assert!(matches!(e.kind, ErrorKind::FieldCount { .. }));
    }

    #[test]
    fn binary_rejected() {
        let e = parse_handling("handling.dat", b"AB\x00CD").unwrap_err();
        assert!(matches!(e.kind, ErrorKind::Binary));
    }
}
