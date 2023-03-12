use std::collections::HashMap;

use crate::d2_enums::{AmmoType, StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, HandlingModifierResponse, MagazineModifierResponse,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverrideResponse, FlinchModifierResponse,
    },
};

pub(super) fn sbr_air_assault(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    let ae_per_stack = if _is_enhanced { 35 } else { 20 };
    let ae = ae_per_stack * _value as i32;
    stats.insert(StatHashes::AIRBORNE.into(), ae);
    stats
}

pub(super) fn fmr_archers_tempo(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_scale: if _value > 0 { 0.75 } else { 1.0 },
        burst_delay_add: 0.0,
        inner_burst_scale: 1.0,
        burst_size_add: 0.0,
    }
}

pub(super) fn dmr_explosive_head(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    if _pvp {
        DamageModifierResponse::new()
    } else {
        DamageModifierResponse {
            impact_dmg_scale: 1.0,
            explosive_dmg_scale: 1.3,
            crit_scale: 1.0,
        }
    }
}

pub(super) fn epr_explosive_head(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ExplosivePercentResponse {
    ExplosivePercentResponse {
        percent: 0.5,
        delyed: if _pvp { 0.0 } else { 0.2 },
        retain_base_total: true,
    }
}

pub(super) fn rsmr_feeding_frenzy(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let val = clamp(_value, 0, 5);
    let duration = 3.5;
    let mut reload_mult = 1.0;
    let mut reload = 0;
    if val == 1 {
        reload = 10;
        reload_mult = 1.0;
    } else if val == 2 {
        reload = 45;
        reload_mult = 0.9;
    } else if val == 3 {
        reload = 55;
        reload_mult = 0.88;
    } else if val == 4 {
        reload = 70;
        reload_mult = 0.85;
    } else if val == 5 {
        reload = 100;
        reload_mult = 0.8;
    };
    if _input.time_total > duration {
        reload = 0;
        reload_mult = 1.0;
    };
    ReloadModifierResponse {
        reload_stat_add: reload,
        reload_time_scale: reload_mult,
    }
}

pub(super) fn sbr_feeding_frenzy(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    let val = clamp(_value, 0, 5);
    let duration = 3.5;
    let mut reload = 0;
    if val == 1 {
        reload = 10;
    } else if val == 2 {
        reload = 45;
    } else if val == 3 {
        reload = 55;
    } else if val == 4 {
        reload = 70;
    } else if val == 5 {
        reload = 100;
    };
    if _input.time_total > duration {
        reload = 0;
    };
    stats.insert(StatHashes::RELOAD.into(), reload);
    stats
}

pub(super) fn dmr_firing_line(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut crit_mult = 1.0;
    if _value > 0 {
        crit_mult = 1.2;
    }
    DamageModifierResponse {
        crit_scale: crit_mult,
        explosive_dmg_scale: 1.0,
        impact_dmg_scale: 1.0,
    }
}

pub(super) fn rr_fourth_times(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RefundResponse {
    RefundResponse {
        crit: true,
        requirement: 4,
        refund_mag: 2,
        refund_reserves: 0,
    }
}

pub(super) fn dmr_killing_tally(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let val = clamp(_value, 0, 3);
    let mut damage_mult = 0.1 * val as f64;
    if _pvp {
        damage_mult *= 0.5;
    };
    if _input.num_reloads > 0.0 {
        damage_mult = 0.0;
    };
    DamageModifierResponse {
        impact_dmg_scale: 1.0 + damage_mult,
        explosive_dmg_scale: 1.0 + damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn mmr_overflow(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> MagazineModifierResponse {
    let mut mag_scale = if _value > 0 { 2.0 } else { 1.0 };
    if _input.total_shots_fired == 0.0 {
        mag_scale = 1.0;
    };
    MagazineModifierResponse {
        magazine_stat_add: 0,
        magazine_scale: mag_scale,
        magazine_add: 0.0,
    }
}

pub(super) fn rsmr_rapid_hit(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let values = vec![
        (0, 1.0),
        (5, 0.99),
        (30, 0.97),
        (35, 0.96),
        (45, 0.94),
        (60, 0.93),
    ];
    let entry_to_get = clamp(_value + _input.shots_fired_this_mag as u32, 0, 5);
    ReloadModifierResponse {
        reload_stat_add: values[entry_to_get as usize].0,
        reload_time_scale: values[entry_to_get as usize].1,
    }
}

pub(super) fn sbr_rapid_hit(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let rel_values = vec![0, 5, 30, 35, 45, 60];
    let stab_values = vec![0, 2, 12, 14, 18, 25];
    let entry_to_get = clamp(_value + _input.shots_fired_this_mag as u32, 0, 5);
    let mut stats = HashMap::new();
    stats.insert(StatHashes::RELOAD.into(), rel_values[entry_to_get as usize]);
    stats.insert(StatHashes::STABILITY.into(), stab_values[entry_to_get as usize]);
    stats
}

pub(super) fn dmr_resevoir_burst(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_mult = 1.0;
    if _input.curr_mag >= _input.base_mag {
        damage_mult = 1.25;
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_surrounded(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_mult = 1.0;
    if _value > 0 {
        damage_mult = if *_input.weapon_type == WeaponType::SWORD {
            1.35
        } else {
            1.4
        };
        if _is_enhanced {
            damage_mult *= 1.05;
        };
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn ror_demolitionist(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadOverrideResponse {
    //todo implement system for cooldown
    let grenade_throw_time = 0.8;
    if _value == 1 {
        return ReloadOverrideResponse {
            valid: true,
            reload_time: _input.handling_data.ready_time + grenade_throw_time,
            ammo_to_reload: _input.base_mag as i32,
            priority: 0,
            count_as_reload: false,
            uses_ammo: true,
        };
    }
    ReloadOverrideResponse::invalid()
}

pub(super) fn dmr_full_court(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_mult = 1.0;
    if _value > 0 {
        damage_mult = 1.25;
    };
    DamageModifierResponse {
        impact_dmg_scale: 1.0,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_swash_buckler(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let val = clamp(_value, 0, 5);
    let duration = if _is_enhanced { 6.0 } else { 4.5 };
    let mut dmg_boost = 0.067 * val as f64;
    if _input.time_total > duration {
        dmg_boost = 0.0;
    };
    DamageModifierResponse {
        impact_dmg_scale: 1.0 + dmg_boost,
        explosive_dmg_scale: 1.0 + dmg_boost,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_multi_kill_clip(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let val = clamp(_value, 0, 5);
    let mut damage_mult = (1.0 / 6.0) * val as f64;
    if _input.num_reloads > 0.0 {
        damage_mult = 0.0;
    };
    DamageModifierResponse {
        impact_dmg_scale: 1.0 + damage_mult,
        explosive_dmg_scale: 1.0 + damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_explosive_light(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let shots = if _is_enhanced { 7.0 } else { 6.0 };
    let shots_left = _value as f64 * shots - _input.total_shots_fired;
    if shots_left <= 0.0 {
        return DamageModifierResponse::new();
    };
    if _input.weapon_type == &WeaponType::GRENADELAUNCHER {
        let blast_radius_struct = _input.stats.get(&StatHashes::BLAST_RADIUS.into());
        let blast_radius;
        if blast_radius_struct.is_none() {
            blast_radius = 0;
        } else {
            blast_radius = blast_radius_struct.unwrap().val();
        };
        if _input.ammo_type == &AmmoType::HEAVY {
            let expl_percent = 0.7 + 0.00175 * blast_radius as f64;
            let impt_percent = 1.0 - expl_percent;
            let expl_mult = 0.875 / expl_percent * 1.6;
            let impt_mult = 0.125 / impt_percent;
            return DamageModifierResponse {
                impact_dmg_scale: impt_mult,
                explosive_dmg_scale: expl_mult,
                crit_scale: 1.0,
            };
        }
        if _input.ammo_type == &AmmoType::SPECIAL {
            let expl_percent = 0.5 + 0.0025 * blast_radius as f64;
            let impt_percent = 1.0 - expl_percent;
            let expl_mult = 0.75 / expl_percent * 1.6;
            let impt_mult = 0.25 / impt_percent;
            return DamageModifierResponse {
                impact_dmg_scale: impt_mult,
                explosive_dmg_scale: expl_mult,
                crit_scale: 1.0,
            };
        }
    };
    DamageModifierResponse {
        explosive_dmg_scale: 1.25,
        impact_dmg_scale: 1.25,
        crit_scale: 1.0,
    }
}

pub(super) fn sbr_explosive_light(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::BLAST_RADIUS.into(), 100);
    };
    out
}

pub(super) fn sbr_eye_of_the_storm(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::HANDLING.into(), 30);
    };
    out
}

pub(super) fn hmr_eye_of_the_storm(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    if _value > 0 {
        HandlingModifierResponse {
            handling_stat_add: 30,
            ..Default::default()
        }
    } else {
        HandlingModifierResponse::default()
    }
}

pub(super) fn flmr_no_distractions(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>)
    -> FlinchModifierResponse
{
    if _value > 0 {
        FlinchModifierResponse  {
            flinch_scale: 0.65,
        }
    } else {
    FlinchModifierResponse::default()
    }
}