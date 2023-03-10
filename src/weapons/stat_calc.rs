use std::collections::HashMap;

use super::{reserve_calc::calc_reserves, Stat, Weapon};
use crate::{
    d2_enums::{StatHashes, WeaponType},
    perks::{
        get_dmg_modifier, get_explosion_data, get_firing_modifier, get_handling_modifier,
        get_magazine_modifier, get_range_modifier, get_reload_modifier, get_reserve_modifier,
        lib::{
            CalculationInput, DamageModifierResponse, FiringModifierResponse,
            HandlingModifierResponse, InventoryModifierResponse, MagazineModifierResponse,
            RangeModifierResponse, ReloadModifierResponse,
        },
    },
    types::rs_types::{
        AmmoFormula, AmmoResponse, FiringResponse, HandlingFormula, HandlingResponse, RangeFormula,
        RangeResponse, ReloadFormula, ReloadResponse,
    },
};

impl ReloadFormula {
    pub fn calc_reload_time_formula(
        &self,
        _reload_stat: i32,
        _modifiers: ReloadModifierResponse,
    ) -> ReloadResponse {
        let reload_stat = if (_reload_stat + _modifiers.reload_stat_add) > 100 {
            100
        } else {
            _reload_stat + _modifiers.reload_stat_add
        } as f64;
        let reload_time = self.reload_data.solve_at(reload_stat) * _modifiers.reload_time_scale;
        ReloadResponse {
            reload_time,
            ammo_time: reload_time * self.ammo_percent,
        }
    }
}
impl Weapon {
    pub fn calc_reload_time(
        &self,
        _calc_input: Option<CalculationInput>,
        _cached_data: Option<&mut HashMap<String, f64>>,
        _pvp: bool,
    ) -> ReloadResponse {
        let reload_stat = self
            .stats
            .get(&StatHashes::RELOAD.into())
            .unwrap_or(&Stat::new())
            .val();
        let mut default_chd_dt = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_chd_dt);
        if _calc_input.is_some() {
            let modifiers = get_reload_modifier(
                self.list_perks(),
                &_calc_input.unwrap(),
                _pvp,
                cached_data,
            );
            self.reload_formula
                .calc_reload_time_formula(reload_stat, modifiers)
        } else {
            self.reload_formula
                .calc_reload_time_formula(reload_stat, ReloadModifierResponse::default())
        }
    }
}

impl RangeFormula {
    pub fn calc_range_falloff_formula(
        &self,
        _range_stat: i32,
        _zoom_stat: i32,
        _modifiers: RangeModifierResponse,
        _floor: f64,
    ) -> RangeResponse {
        let range_stat = if (_range_stat + _modifiers.range_stat_add) > 100 {
            100
        } else {
            _range_stat + _modifiers.range_stat_add
        } as f64;
        let zoom_stat = _zoom_stat as f64 * _modifiers.range_zoom_scale;

        let zoom_mult = if self.fusion {
            1.0 + 0.02 * zoom_stat
        } else {
            0.1 * zoom_stat - 0.025
        };

        let mut hip_falloff_start = self.start.solve_at(range_stat) * _modifiers.range_all_scale;
        let mut hip_falloff_end = self.end.solve_at(range_stat) * _modifiers.range_all_scale;

        let ads_falloff_start = hip_falloff_start * zoom_mult;
        let ads_falloff_end = hip_falloff_end * zoom_mult;

        hip_falloff_start *= _modifiers.range_hip_scale;
        hip_falloff_end *= _modifiers.range_hip_scale;

        RangeResponse {
            hip_falloff_start,
            hip_falloff_end,
            ads_falloff_start,
            ads_falloff_end,
            floor_percent: _floor,
        }
    }
}
impl Weapon {
    pub fn calc_range_falloff(
        &self,
        _calc_input: Option<CalculationInput>,
        _cached_data: Option<&mut HashMap<String, f64>>,
        _pvp: bool,
    ) -> RangeResponse {
        let range_stat = self
            .stats
            .get(&StatHashes::RANGE.into())
            .unwrap_or(&Stat::new())
            .val();
        let zoom_stat = self
            .stats
            .get(&StatHashes::ZOOM.into())
            .unwrap_or(&Stat::new())
            .val();
        let mut default_chd_dt = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_chd_dt);
        if _calc_input.is_some() {
            let modifiers = get_range_modifier(
                self.list_perks(),
                &_calc_input.unwrap(),
                _pvp,
                cached_data,
            );
            self.range_formula.calc_range_falloff_formula(
                range_stat,
                zoom_stat,
                modifiers,
                self.range_formula.floor_percent,
            )
        } else {
            self.range_formula.calc_range_falloff_formula(
                range_stat,
                zoom_stat,
                RangeModifierResponse::default(),
                self.range_formula.floor_percent,
            )
        }
    }
}

impl HandlingFormula {
    pub fn calc_handling_times_formula(
        &self,
        _handling_stat: i32,
        _modifiers: HandlingModifierResponse,
    ) -> HandlingResponse {
        let handling_stat = if (_handling_stat + _modifiers.handling_stat_add) > 100 {
            100
        } else {
            _handling_stat + _modifiers.handling_stat_add
        } as f64;
        let ready_time = self.ready.solve_at(handling_stat) * _modifiers.handling_swap_scale;
        let mut stow_time = self.stow.solve_at(handling_stat) * _modifiers.handling_swap_scale;
        let ads_time = self.ads.solve_at(handling_stat) * _modifiers.handling_ads_scale;
        if stow_time < self.stow.solve_at(100.0) {
            stow_time = self.stow.solve_at(100.0);
        }
        HandlingResponse {
            ready_time,
            stow_time,
            ads_time,
        }
    }
}
impl Weapon {
    pub fn calc_handling_times(
        &self,
        _calc_input: Option<CalculationInput>,
        _cached_data: Option<&mut HashMap<String, f64>>,
        _pvp: bool,
    ) -> HandlingResponse {
        let handling_stat = self
            .stats
            .get(&StatHashes::HANDLING.into())
            .unwrap_or(&Stat::new())
            .val();
        let mut default_chd_dt = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_chd_dt);
        if _calc_input.is_some() {
            let modifiers = get_handling_modifier(
                self.list_perks(),
                &_calc_input.unwrap(),
                _pvp,
                cached_data,
            );
            self.handling_formula
                .calc_handling_times_formula(handling_stat, modifiers)
        } else {
            self.handling_formula
                .calc_handling_times_formula(handling_stat, HandlingModifierResponse::default())
        }
    }
}

impl AmmoFormula {
    pub fn calc_ammo_size_formula(
        &self,
        _mag_stat: i32,
        _mag_modifiers: MagazineModifierResponse,
        _reserve_stat: i32,
        _inv_modifiers: InventoryModifierResponse,
        _calc_inv: bool,
        _inv_id: u32,
    ) -> AmmoResponse {
        let mag_stat = if (_mag_stat + _mag_modifiers.magazine_stat_add) > 100 {
            100
        } else {
            _mag_stat + _mag_modifiers.magazine_stat_add
        } as f64;

        let inv_stat = if (_reserve_stat + _inv_modifiers.inv_stat_add) > 100 {
            100
        } else {
            _reserve_stat + _inv_modifiers.inv_stat_add
        } as f64;

        let raw_mag_size =
            (self.mag.evpp * (mag_stat.powi(2))) + (self.mag.vpp * mag_stat) + self.mag.offset;

        let mut mag_size =
            (((self.mag.evpp * (mag_stat.powi(2))) + (self.mag.vpp * mag_stat) + self.mag.offset)
                .ceil()
                * _mag_modifiers.magazine_scale
                + _mag_modifiers.magazine_add)
                .ceil() as i32;
        if mag_size < 1 {
            mag_size = 1;
        }

        let mut reserve_size = 1;
        if _calc_inv {
            reserve_size = calc_reserves(raw_mag_size, _mag_stat as i32, inv_stat as i32, _inv_id);
        }
        AmmoResponse {
            mag_size,
            reserve_size,
        }
    }
}
impl Weapon {
    pub fn calc_ammo_sizes(
        &self,
        _calc_input: Option<CalculationInput>,
        _cached_data: Option<&mut HashMap<String, f64>>,
        _pvp: bool,
    ) -> AmmoResponse {
        let mag_stat = self
            .stats
            .get(&StatHashes::MAGAZINE.into())
            .unwrap_or(&Stat::new())
            .val();
        let inv_stat = self
            .stats
            .get(&StatHashes::INVENTORY_SIZE.into())
            .unwrap_or(&Stat::new())
            .val();
        let mut out;
        let mut default_chd_dt = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_chd_dt);
        if _calc_input.is_some() {
            let mag_modifiers = get_magazine_modifier(
                self.list_perks(),
                &_calc_input.clone().unwrap(),
                _pvp,
                cached_data,
            );
            let inv_modifiers = get_reserve_modifier(
                self.list_perks(),
                &_calc_input.clone().unwrap(),
                _pvp,
                cached_data,
            );
            out = self.ammo_formula.calc_ammo_size_formula(
                mag_stat,
                mag_modifiers,
                inv_stat,
                inv_modifiers,
                true,
                self.ammo_formula.reserve_id,
            );
        } else {
            out = self.ammo_formula.calc_ammo_size_formula(
                mag_stat,
                MagazineModifierResponse::default(),
                inv_stat,
                InventoryModifierResponse::default(),
                true,
                self.ammo_formula.reserve_id,
            );
        }
        if mag_stat > 90 && self.weapon_type == WeaponType::SNIPER {
            out.mag_size += 1;
        }
        out
    }
}

impl Weapon {
    pub fn calc_firing_data(
        &self,
        _calc_input: Option<CalculationInput>,
        _cached_data: Option<&mut HashMap<String, f64>>,
        _pvp: bool,
    ) -> FiringResponse {
        let pve_damage_modifiers: DamageModifierResponse;
        let pvp_damage_modifiers: DamageModifierResponse;
        let firing_modifiers: FiringModifierResponse;
        let mut default_cached_data = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_cached_data);
        if _calc_input.is_some() {
            firing_modifiers = get_firing_modifier(
                self.list_perks(),
                &_calc_input.clone().unwrap(),
                _pvp,
                cached_data,
            );
            pvp_damage_modifiers = get_dmg_modifier(
                self.list_perks(),
                &_calc_input.clone().unwrap(),
                true,
                cached_data,
            );
            pve_damage_modifiers = get_dmg_modifier(
                self.list_perks(),
                &_calc_input.clone().unwrap(),
                false,
                cached_data,
            );
        } else {
            firing_modifiers = FiringModifierResponse::default();
            pvp_damage_modifiers = DamageModifierResponse::new();
            pve_damage_modifiers = DamageModifierResponse::new();
        };
        let tmp_dmg_prof = self.get_damage_profile();
        let impact_dmg = tmp_dmg_prof.0;
        let explosion_dmg = tmp_dmg_prof.1;
        let crit_mult = tmp_dmg_prof.2;

        let fd = self.firing_data;
        let extra_charge_delay = if self.weapon_type == WeaponType::FUSIONRIFLE {
            0.45
        } else if self.weapon_type == WeaponType::LINEARFUSIONRIFLE {
            0.95
        } else {
            0.0
        };
        let burst_delay = (fd.burst_delay + firing_modifiers.burst_delay_add)
            * firing_modifiers.burst_delay_scale;
        let burst_size = fd.burst_size + firing_modifiers.burst_size_add as i32;
        let inner_burst_delay = fd.inner_burst_delay * firing_modifiers.inner_burst_scale;
        let raw_rpm = 60.0
            / ((burst_delay
                + (inner_burst_delay * (burst_size as f64 - 1.0))
                + extra_charge_delay)
                / burst_size as f64);
        let rpm: f64;
        if self.firing_data.one_ammo {
            rpm = raw_rpm / burst_size as f64
        } else {
            rpm = raw_rpm
        };
        let out = FiringResponse {
            pvp_impact_damage: impact_dmg * pvp_damage_modifiers.impact_dmg_scale,
            pvp_explosion_damage: explosion_dmg * pvp_damage_modifiers.explosive_dmg_scale,
            pvp_crit_mult: crit_mult * pvp_damage_modifiers.crit_scale,

            pve_impact_damage: impact_dmg * pve_damage_modifiers.impact_dmg_scale,
            pve_explosion_damage: explosion_dmg * pve_damage_modifiers.explosive_dmg_scale,
            pve_crit_mult: crit_mult * pve_damage_modifiers.crit_scale,

            burst_delay,
            burst_size,
            inner_burst_delay,

            rpm,
        };
        out
    }
}

impl Weapon {
    pub fn get_damage_profile(&self) -> (f64, f64, f64, f64) {
        let impact;
        let mut explosion = 0.0_f64;
        let mut crit = 1.0_f64;
        let delay;

        let epr = get_explosion_data(self.list_perks(), &self.static_calc_input(), false);
        if epr.percent <= 0.0 {
            impact = self.firing_data.damage;
            crit = self.firing_data.crit_mult;
            delay = 0.0;
        } else {
            impact = self.firing_data.damage * (1.0 - epr.percent);
            explosion = self.firing_data.damage * epr.percent;
            if epr.retain_base_total && self.firing_data.crit_mult > 1.0 {
                crit = (self.firing_data.crit_mult - 1.0) / (1.0 - epr.percent) + 1.0
            }
            delay = epr.delyed;
        }
        (impact, explosion, crit, delay)
    }
}
