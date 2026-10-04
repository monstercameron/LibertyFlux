//! Weapon tables: `WeaponInfo.xml` and `ThrownWeaponInfo.xml`.
//!
//! `WeaponInfo.xml` (root `<weaponinfo version="1">`) holds one `<weapon
//! type="...">` element per weapon. Each weapon has a `<data>` element
//! (slot, fire type, damage type, group, ranges, clip size, ammo, timing),
//! a `<damage>` element, `<pickup>`, `<controller>`/`<rumble>`, `<flags>`
//! (a list of `<flag>` words), and a varying set of optional blocks:
//! `<aiming>`, `<anim>`, `<assets>`, `<crouchedoffset>`, `<effects>`,
//! `<explosion>`, `<muzzle>`, `<offset>`, `<physics>`, `<projectile>`,
//! `<rates>`, `<reload>`, `<reticule>`, `<rotoffset>`, `<shell>`, `<trail>`,
//! `<typetocreate>`. Optional blocks are kept generically (name plus
//! attributes) so the parse is lossless without a struct per block.
//!
//! `ThrownWeaponInfo.xml` (root `<thrownweaponinfo>`) holds `<object
//! name="...">` elements with `<offset>` and `<rotoffset>` children.

use crate::xml::{Element, parse_xml};
use crate::{Error, ErrorKind, Result};

/// One named sub-block kept generically (name + attributes + flags/text).
#[derive(Debug, Clone)]
pub struct WeaponBlock {
    /// Element name (`physics`, `reload`, ...).
    pub name: String,
    /// Attributes in file order.
    pub attrs: Vec<(String, String)>,
    /// Text of any `<flag>`-style children.
    pub words: Vec<String>,
}

impl WeaponBlock {
    fn from_element(el: &Element) -> Self {
        let words = el.children.iter().filter_map(|c| c.text.clone()).collect();
        Self {
            name: el.name.clone(),
            attrs: el.attrs.clone(),
            words,
        }
    }

    /// Attribute value by name, if present.
    #[must_use]
    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }
}

/// One weapon: type plus core blocks and generic extras.
#[derive(Debug, Clone)]
pub struct Weapon {
    /// Weapon type (`UNARMED`, `PISTOL`, ...).
    pub weapon_type: String,
    /// The `<data>` element (slot, firetype, damagetype, group, ...),
    /// if the weapon has one. Two shipped weapons (projectile and
    /// placeholder types) omit it.
    pub data: Option<WeaponBlock>,
    /// The `<damage>` element, if present.
    pub damage: Option<WeaponBlock>,
    /// The `<pickup>` element, if present.
    pub pickup: Option<WeaponBlock>,
    /// Flag words from `<flags>`.
    pub flags: Vec<String>,
    /// Every other child block, in file order.
    pub extra: Vec<WeaponBlock>,
}

impl Weapon {
    fn from_element(file: &str, el: &Element) -> Result<Self> {
        let weapon_type = el.attr("type").ok_or_else(|| {
            Error::whole_file(
                file,
                ErrorKind::BadHeader {
                    want: "weapon type attribute",
                },
            )
        })?;
        let data = el.child("data").map(WeaponBlock::from_element);
        let mut extra = Vec::new();
        let mut flags = Vec::new();
        let mut damage = None;
        let mut pickup = None;
        for c in &el.children {
            match c.name.as_str() {
                "data" => {}
                "damage" => damage = Some(WeaponBlock::from_element(c)),
                "pickup" => pickup = Some(WeaponBlock::from_element(c)),
                "flags" => {
                    for f in c.children_named("flag") {
                        if let Some(t) = &f.text {
                            flags.push(t.clone());
                        }
                    }
                }
                _ => extra.push(WeaponBlock::from_element(c)),
            }
        }
        Ok(Self {
            weapon_type: weapon_type.to_string(),
            data,
            damage,
            pickup,
            flags,
            extra,
        })
    }
}

/// Full contents of a `WeaponInfo.xml` file.
#[derive(Debug, Clone)]
pub struct WeaponInfo {
    /// Root `version` attribute.
    pub version: String,
    /// Weapons in file order.
    pub weapons: Vec<Weapon>,
}

impl WeaponInfo {
    /// Find a weapon by type.
    #[must_use]
    pub fn weapon(&self, weapon_type: &str) -> Option<&Weapon> {
        self.weapons.iter().find(|w| w.weapon_type == weapon_type)
    }
}

/// Parse a `WeaponInfo.xml` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_weapon_info(file: &str, bytes: &[u8]) -> Result<WeaponInfo> {
    let root = parse_xml(file, bytes)?;
    if root.name != "weaponinfo" {
        return Err(Error::whole_file(
            file,
            ErrorKind::BadHeader {
                want: "weaponinfo root",
            },
        ));
    }
    let version = root.attr("version").unwrap_or("").to_string();
    let mut weapons = Vec::new();
    for w in root.children_named("weapon") {
        weapons.push(Weapon::from_element(file, w)?);
    }
    Ok(WeaponInfo { version, weapons })
}

/// One thrown-weapon object: grip offsets.
#[derive(Debug, Clone)]
pub struct ThrownObject {
    /// Object name.
    pub name: String,
    /// Grip offset.
    pub offset: [f32; 3],
    /// Grip rotation offset.
    pub rot_offset: [f32; 3],
}

/// Parse a `ThrownWeaponInfo.xml` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_thrown_weapon_info(file: &str, bytes: &[u8]) -> Result<Vec<ThrownObject>> {
    let root = parse_xml(file, bytes)?;
    if root.name != "thrownweaponinfo" {
        return Err(Error::whole_file(
            file,
            ErrorKind::BadHeader {
                want: "thrownweaponinfo root",
            },
        ));
    }
    let mut out = Vec::new();
    for obj in root.children_named("object") {
        let name = obj.attr("name").ok_or_else(|| {
            Error::whole_file(
                file,
                ErrorKind::BadHeader {
                    want: "object name attribute",
                },
            )
        })?;
        let offset_el = obj.child("offset");
        let rot_el = obj.child("rotoffset");
        let get = |el: Option<&Element>| -> Result<[f32; 3]> {
            let el = el.ok_or_else(|| {
                Error::whole_file(
                    file,
                    ErrorKind::BadHeader {
                        want: "offset element",
                    },
                )
            })?;
            let x = el.attr_f32(file, "x")?.unwrap_or(0.0);
            let y = el.attr_f32(file, "y")?.unwrap_or(0.0);
            let z = el.attr_f32(file, "z")?.unwrap_or(0.0);
            Ok([x, y, z])
        };
        out.push(ThrownObject {
            name: name.to_string(),
            offset: get(offset_el)?,
            rot_offset: get(rot_el)?,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "<weaponinfo version=\"1\"><weapon type=\"PISTOL\"><data slot=\"PISTOL\" firetype=\"INSTANT\" damagetype=\"BULLET\" group=\"SMALL\" targetrange=\"120.0\" weaponrange=\"40.0\" clipsize=\"12\"/><damage base=\"10\" networkplayermod=\"1.0\" networkpedmod=\"1.0\"/><pickup regentime=\"30\"/><controller><rumble duration=\"60\" intensity=\"0.23\"/></controller><flags><flag>CAN_AIM</flag><flag>GUN</flag></flags><reload time=\"1.0\" fasttime=\"0.8\" crouchtime=\"1.2\"/></weapon></weaponinfo>";

    #[test]
    fn parses_weapon() {
        let w = parse_weapon_info("WeaponInfo.xml", SAMPLE.as_bytes()).unwrap();
        assert_eq!(w.version, "1");
        assert_eq!(w.weapons.len(), 1);
        let p = &w.weapons[0];
        assert_eq!(p.weapon_type, "PISTOL");
        assert_eq!(p.data.as_ref().unwrap().attr("clipsize"), Some("12"));
        assert_eq!(p.flags, vec!["CAN_AIM", "GUN"]);
        assert_eq!(p.extra.len(), 2); // controller, reload
    }

    #[test]
    fn parses_thrown() {
        let src = "<thrownweaponinfo><object name=\"CJ_PROC_Tin\"><offset x=\"0.11\" y=\"0.04\" z=\"0\"/><rotoffset x=\"0\" y=\"90\" z=\"0\"/></object></thrownweaponinfo>";
        let o = parse_thrown_weapon_info("ThrownWeaponInfo.xml", src.as_bytes()).unwrap();
        assert_eq!(o.len(), 1);
        assert_eq!(o[0].rot_offset[1], 90.0);
    }

    #[test]
    fn wrong_root_is_error() {
        let e = parse_weapon_info("WeaponInfo.xml", b"<other/>").unwrap_err();
        assert!(matches!(e.kind, ErrorKind::BadHeader { .. }));
    }
}
