pub mod dps_calc;
pub mod reserve_calc;
pub mod stat_calc;
pub mod ttk_calc;
pub mod weapon_formulas;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::d2_enums::{AmmoType, DamageType, StatHashes, WeaponType};
use crate::enemies::Enemy;
use crate::perks::{
    get_magazine_modifier, get_perk_stats, get_reserve_modifier, lib::CalculationInput, Perk,
};

use crate::types::rs_types::{
    AmmoFormula, DamageMods, DpsResponse, HandlingFormula, RangeFormula, ReloadFormula,
};

use self::dps_calc::complex_dps_calc;

#[derive(Debug, Clone)]
pub struct PsuedoWeapon {}

#[derive(Debug, Clone, Serialize)]
pub struct Stat {
    pub base_value: i32,
    pub part_value: i32,
    pub perk_value: i32,
}
impl Stat {
    pub fn new() -> Stat {
        Stat {
            base_value: 0,
            part_value: 0,
            perk_value: 0,
        }
    }
    pub fn val(&self) -> i32 {
        self.base_value + self.part_value
    }
    pub fn perk_val(&self) -> i32 {
        self.base_value + self.part_value + self.perk_value
    }
}
impl From<i32> for Stat {
    fn from(_val: i32) -> Self {
        Stat {
            base_value: _val,
            part_value: 0,
            perk_value: 0,
        }
    }
}

#[derive(Debug, Clone, Default, Copy, Deserialize)]
pub struct FiringConfig {
    pub damage: f64,
    pub crit_mult: f64,
    pub burst_delay: f64,
    pub burst_duration: f64,
    pub burst_size: i32,
    pub one_ammo: bool,
    pub charge: bool,
    pub explosive: bool,
}

#[derive(Debug, Clone)]
pub struct Weapon {
    //ideally entirely interfaced with through funcs when acting mutably
    pub is_pvp: bool,
    pub hash: u32,

    pub perks: HashMap<u32, Perk>,
    pub stats: HashMap<u32, Stat>,

    pub damage_mods: DamageMods,
    pub firing_data: FiringConfig,
    pub range_formula: RangeFormula,
    pub ammo_formula: AmmoFormula,
    pub handling_formula: HandlingFormula,
    pub reload_formula: ReloadFormula,

    pub weapon_type: WeaponType,
    pub damage_type: DamageType,
    pub ammo_type: AmmoType,
}
impl Weapon {
    pub fn add_perk(&mut self, _perk: Perk) {
        self.perks.insert(_perk.hash, _perk);
        self.update_stats();
    }
    pub fn remove_perk(&mut self, _perk_hash: u32) {
        self.perks.remove(&_perk_hash);
        self.update_stats();
    }
    pub fn list_perk_ids(&self) -> Vec<u32> {
        let mut perk_list: Vec<u32> = Vec::new();
        for (key, _perk) in &self.perks {
            perk_list.push(*key);
        }
        perk_list
    }
    pub fn list_perks(&self) -> Vec<Perk> {
        let mut perk_list: Vec<Perk> = Vec::new();
        for (_key, perk) in &self.perks {
            perk_list.push(perk.clone());
        }
        perk_list
    }
    pub fn change_perk_val(&mut self, _perk_hash: u32, _val: u32) {
        let perk_opt = self.perks.get_mut(&_perk_hash);
        if perk_opt.is_some() {
            perk_opt.unwrap().value = _val;
        }
        self.update_stats();
    }
    pub fn get_stats(&mut self) -> HashMap<u32, Stat> {
        self.update_stats();
        self.stats.clone()
    }
    pub fn set_stats(&mut self, _stats: HashMap<u32, Stat>) {
        self.stats = _stats;
        self.update_stats()
    }
    pub fn reset(&mut self) {
        self.perks = HashMap::new();
        self.stats = HashMap::new();
        self.hash = 0;
        self.damage_mods = DamageMods::default();
        self.firing_data = FiringConfig::default();
        self.range_formula = RangeFormula::default();
        self.ammo_formula = AmmoFormula::default();
        self.handling_formula = HandlingFormula::default();
        self.reload_formula = ReloadFormula::default();
    }

    pub fn static_calc_input(&self) -> CalculationInput {
        CalculationInput::construct_static(
            &self.firing_data,
            &self.stats,
            &self.weapon_type,
            &self.ammo_type,
        )
    }

    pub fn sparse_calc_input(&self, _total_shots_fired: i32, _total_time: f64) -> CalculationInput {
        CalculationInput::construct_pve_sparse(
            &self.firing_data,
            &self.stats,
            &self.weapon_type,
            &self.ammo_type,
            &self.damage_type,
            self.firing_data.damage,
            self.firing_data.crit_mult,
            self.calc_ammo_sizes(None).mag_size,
            _total_shots_fired,
            _total_time,
        )
    }
    pub fn pvp_calc_input(&self, _total_shots_fired: f64, _total_shots_hit: f64, _total_time: f64, _overshield: bool) -> CalculationInput {
        let base_mag = self.calc_ammo_sizes(None).mag_size as f64;
        let mut tmp = CalculationInput::construct_pvp(
            &self.firing_data, 
            &self.stats, 
            &self.weapon_type, 
            &self.ammo_type, 
            self.firing_data.damage, 
            self.firing_data.crit_mult,
            base_mag, 
            _overshield, 
            self.calc_handling_times(None)
        );
        tmp.time_this_mag = _total_time; 
        tmp.time_total = _total_time;
        tmp.shots_fired_this_mag = _total_shots_fired;
        tmp.total_shots_fired = _total_shots_fired;
        tmp.total_shots_hit = _total_shots_hit;
        tmp
    }
    pub fn update_stats(&mut self) {
        let input = CalculationInput::construct_static(
            &self.firing_data,
            &self.stats,
            &self.weapon_type,
            &self.ammo_type,
        );
        let inter_var = get_perk_stats(self.list_perks(), input, false);
        let dynamic_stats = &inter_var[0];
        let static_stats = &inter_var[1];
        for (key, stat) in &mut self.stats {
            let a = static_stats.get(key);
            let b = dynamic_stats.get(key);
            if a.is_some() {
                stat.part_value = a.unwrap().clone();
            }
            if b.is_some() {
                stat.perk_value = b.unwrap().clone();
            }
        }
    }
    pub fn calc_dps(&self, _enemy: Enemy, _pl_dmg_mult: f64) -> DpsResponse {
        complex_dps_calc(self.clone(), _enemy, _pl_dmg_mult)
    }
}
impl Default for Weapon {
    fn default() -> Weapon {
        Weapon {
            is_pvp: false,

            perks: HashMap::new(),
            stats: HashMap::new(),
            hash: 0,
            damage_mods: DamageMods::default(),
            firing_data: FiringConfig::default(),

            range_formula: RangeFormula::default(),
            ammo_formula: AmmoFormula::default(),
            handling_formula: HandlingFormula::default(),
            reload_formula: ReloadFormula::default(),

            weapon_type: WeaponType::UNKNOWN,
            damage_type: DamageType::UNKNOWN,
            ammo_type: AmmoType::UNKNOWN,
        }
    }
}
