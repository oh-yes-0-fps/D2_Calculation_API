#![allow(clippy::all)]

pub mod enhanced_handler;
pub mod exotic_perks;
pub mod lib;
pub mod meta_perks;
pub mod origin_perks;
pub mod other_perks;
pub mod perk_options_handler;
pub mod year_1_perks;
pub mod year_2_perks;
pub mod year_3_perks;
pub mod year_4_perks;
pub mod year_5_perks;
pub mod year_6_perks;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::d2_enums::StatHashes;

use self::{
    exotic_perks::*,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, HandlingModifierResponse, InventoryModifierResponse,
        MagazineModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        ReloadOverrideResponse, FlinchModifierResponse,
    },
    meta_perks::*,
    origin_perks::*,
    other_perks::*,
    year_1_perks::*,
    year_2_perks::*,
    year_3_perks::*,
    year_4_perks::*,
    year_5_perks::*,
    year_6_perks::*,
};

pub fn clamp<T: PartialOrd>(n: T, min: T, max: T) -> T {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Perk {
    pub stat_buffs: HashMap<u32, i32>,
    pub enhanced: bool,
    pub value: u32, //used for toggle and stacks
    pub hash: u32,
}

// all armor pekrs are for the future but wanted to started to compile them now
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Perks {
    ////TOGGLE////
    VeistStinger, //will give it 100% chance
    Surrounded,
    Harmony,
    Frenzy,       //if its disabled will still activate after 12s in dps anyways
    HakkeBreach,  // can't check if this is a viable option so will always allow it
    CloseToMelee, //such a stupid name
    SteadyHands,
    Cornered,
    KillClip,
    Ensemble,
    FiringLine,
    StatsForAll,
    FragileFocus,
    KillingWind,
    Desperado,
    CascadePoint,
    Outlaw,
    BackupPlan,   // will apply in dps no matter what
    BoxBreathing, // will apply in dps no matter what
    Pugilist,
    WellRounded,
    ExplosiveLight,
    Adagio,
    Ambush,
    FieldPrep,
    OpeningShot,
    FirmlyPlanted,
    SlideShot,
    SlideWays,
    TapTheTrigger,
    PerfectFloat,
    OffhandStrike,
    TakenSpec,
    OverFlow,
    Reconstruction,
    UnderPressure,
    TrenchBarrel,
    DisruptionBreak,
    UnderDog,
    OneForAll,
    FireFly,
    DangerZone,
    TunnelVision,
    Alacrity,
    FluidDynamics,
    QuietMoment,
    SurosSynergy,
    CompulsiveReloader,
    BaitAndSwitch,
    UnstoppableForce,
    HotSwap,
    RightHook,
    KeepAway,
    NoDistractions,
    //class
    Amplified,
    Tempering,
    HeatRises,
    Hedrons,
    Frequency,
    FlowState,

    ////SLIDER////
    FeedingFrenzy,
    RunnethOver,
    MultikillClip,
    Encore,
    SuccesfulWarmup,
    Swashbuckler,
    Surplus,
    RapidHit, // dps will still start at 0 :)
    PerpetualMotion,
    AdrenalineJunkie,
    Rampage,
    ThreatDetector,
    AirAssault,
    KillingTally,
    AmbitiousAssassin,
    FullCourt,
    Recombination,
    HeatingUp,
    GoldenTricorn,
    SleightOfHand,
    BitterSpite,
    TexBalancedStock,
    ShotSwap,
    //class
    OnYourMark,
    //weird
    Demolitionist,
    ElementalCapacitor,

    //armor
    //slides between 1 and 2
    DexterityMod,
    ReloadMod,
    ReserveMod,
    TargetingMod,
    LoaderMod,
    UnflinchingMod,

    ////STATIC////
    GutShot,
    Vorpal,
    ImpulseAmplifier,
    MovingTarget,
    TargetLock,
    HighImpactReserves,
    FocusedFury,
    ChillClip,
    LastingImpression,
    TripleTap,
    FourthTimesTheCharm,
    HipFireGrip,
    RewindRounds,
    ExplosivePayload,
    TimedPayload,
    ExplosiveHead,
    SpikeGrenades,
    DisorientingGrenades,
    AlloyMag,
    RapidFireFrame,
    SwapMag,
    QuickAccessSling,
    BossSpec,
    MinorSpec,
    MajorSpec,
    BigOnesSpec,
    AssaultMag,
    QuickDraw,
    ImpactCasing,
    FullChoke,
    ResevoirBurst,
    OverUnder,
    ArchersTempo,
    Snapshot,
    Slickdraw,
    ClownCartridge,
    RangeFinder,
    ClusterBomb,
    FullAutoTrigger,
    HeadSeeker,
    DualLoader,
    SearchParty,
    AcceleratedCoils,
    LiquidCoils,
    ChargetimeMW,
    AdeptChargeTime,

    //armor
    QuickCharge,

    ////MISC////
    Ignore,
    MasterWork,
    BuiltIn,
    EmpowermentBuffs,
    WeakenDebuffs,
    RallyBarricade,
    ////////EXOTIC////////
    ////TOGGLE////
    CranialSpike,
    AgersCall,
    LagragianSight,
    OphidianAspect,
    DragonShadow,
    LunaFaction,
    TomeOfDawn,

    ////SLIDER////
    RatPack,
    StringofCurses,
    WormsHunger,

    ////STATIC////
    RocketTracers,
    ParacausalShot,
    CorruptionSpreads,
    TimeSlip,
    ToM,
    IgnitionTrigger,
    GuidanceRing,
    ConserveMomentum,
    Impetus,
    LooksCanKill,
    PerfectFith,
    Broadside,
    Stormbringer,
    PrismaticInferno,
    ReignHavoc,
    WhisperCatalyst,
    Roadborn,
    HakkeHeavyBurst,

    //too lazy to sort
    SwoopingTalons,
    CalculatedBalance,
    RavenousBeast,
    LordOfWolvesCatalyst,
    ReleaseTheWolves,
    Fundamentals,
    ThinTheHerd,
    Chimera,
    FirstGlance,
    FateOfAllFools,
    HonedEdge,
    TakenPredator,
    MarkovChain,
    StormAndStress,
    DualSpeedReceiver,
    ExplosiveShadow,
    SurosLegacy,
    SpinningUp,
    DarkDescent,
    SleeperCatalyst,
    TargetAquired,
    PulseMonitor,
    EyeOfTheStorm,
    FullStop,
    RideTheBull,
    HuntersTrance, //hawkmoon catalyst not R0
    FasterStringT1,
    FasterStringT2,
    SlowerStringT1,
    SlowerStringT2,
    FieldTested,
}

impl From<u32> for Perks {
    fn from(_value: u32) -> Perks {
        match _value {
            //Meta perks
            0 => Perks::BuiltIn,
            222 => Perks::EmpowermentBuffs,
            333 => Perks::WeakenDebuffs,
            444 => Perks::RallyBarricade,

            //intrinsics
            902 => Perks::RapidFireFrame,

            //armor
            111111111 => Perks::DexterityMod,
            222222222 => Perks::TargetingMod,
            333333333 => Perks::ReserveMod,
            444444444 => Perks::LoaderMod,
            555555555 => Perks::UnflinchingMod,
            1484685884 => Perks::QuickCharge,
            593361144 => Perks::DragonShadow,
            1147638875 => Perks::OphidianAspect,
            3347978672 => Perks::LunaFaction,
            926349160 => Perks::TomeOfDawn,

            //parts
            3796465595 => Perks::ImpactCasing,
            3721627275 => Perks::SwapMag,
            1047830412 => Perks::FullChoke,
            3301904089 => Perks::SpikeGrenades,
            1431678320 => Perks::AlloyMag,
            1687452232 => Perks::LiquidCoils,
            689005463 => Perks::AcceleratedCoils,
            3128594062 => Perks::ChargetimeMW,
            3032599245 => Perks::DisorientingGrenades,
            791862061 => Perks::AssaultMag,
            //bow strings
            3371775011 => Perks::SlowerStringT1,
            852209214 => Perks::FasterStringT1,
            4067834857 => Perks::FasterStringT1,
            2801223209 => Perks::FasterStringT2,
            1885045197 => Perks::FasterStringT1,

            //mods
            1334978104 => Perks::QuickAccessSling,
            2788909693 => Perks::BossSpec,
            984527513 => Perks::MajorSpec,
            4091000557 => Perks::MinorSpec,
            3018373291 => Perks::BigOnesSpec,
            1513326571 => Perks::TakenSpec,
            744770875 => Perks::AdeptChargeTime,

            //origin | year 5+
            3988215619 => Perks::VeistStinger,
            1607056502 => Perks::HakkeBreach,
            2988596335 => Perks::Alacrity,
            2839173408 => Perks::FluidDynamics,
            4091460919 => Perks::QuietMoment,
            4008116374 => Perks::SurosSynergy,
            4154828211 => Perks::BitterSpite,
            120721526 => Perks::RunnethOver,
            1260401931 => Perks::HotSwap,
            3907865655 => Perks::RightHook,
            192157151 => Perks::Ambush,
            2437618208 => Perks::TexBalancedStock,
            2250679103 => Perks::SearchParty,
            2120661319 => Perks::FieldTested,

            //season 1 | year 1
            1015611457 => Perks::KillClip,
            1168162263 => Perks::Outlaw,
            1528281896 => Perks::Outlaw, //rose
            3124871000 => Perks::Outlaw, //redrix
            1266037487 => Perks::Outlaw, //R0
            1600092898 => Perks::BackupPlan,
            2869569095 => Perks::FieldPrep,
            3425386926 => Perks::Rampage,
            3551326236 => Perks::Rampage, //huckleberry
            47981717 => Perks::OpeningShot,
            588594999 => Perks::MovingTarget,
            2010801679 => Perks::AmbitiousAssassin,
            1275731761 => Perks::ClusterBomb,
            3038247973 => Perks::ExplosivePayload,
            280464955 => Perks::FirmlyPlanted,
            2117683199 => Perks::FullAutoTrigger,
            460017080 => Perks::HeadSeeker,
            2213355989 => Perks::HighImpactReserves,
            1866048759 => Perks::HipFireGrip,
            957782887 => Perks::Snapshot,
            1890422124 => Perks::TapTheTrigger,
            2039302152 => Perks::SlideWays,
            706527188 => Perks::QuickDraw,
            1954620775 => Perks::TimedPayload,
            4071163871 => Perks::ThreatDetector,
            3161816588 => Perks::SlideShot,
            3400784728 => Perks::TripleTap,
            1409312565 => Perks::TripleTap, //cloudstrike
            1645158859 => Perks::UnderPressure,
            972757866 => Perks::PulseMonitor,

            //season 2 | year 1
            //lmao bozo

            //season 3 | year 1
            2846385770 => Perks::RangeFinder,
            3871884143 => Perks::DisruptionBreak,
            1683379515 => Perks::DisruptionBreak,
            806159697 => Perks::TrenchBarrel,
            2360754333 => Perks::TrenchBarrel,
            3047969693 => Perks::Desperado,
            2551157718 => Perks::BoxBreathing,

            //season 4 | year 2
            201365942 => Perks::ArchersTempo,
            3365897133 => Perks::ExplosiveHead,
            2779035018 => Perks::FeedingFrenzy,
            1266037485 => Perks::FeedingFrenzy, //R0
            1354429876 => Perks::FourthTimesTheCharm,
            1266037486 => Perks::FourthTimesTheCharm, //R0
            247725512 => Perks::RapidHit,

            //season 5 | year 2
            1427256713 => Perks::ResevoirBurst,
            3708227201 => Perks::Surrounded,
            3722653512 => Perks::AirAssault,
            3643424744 => Perks::OverFlow,

            //season 6 | year 2
            1771339417 => Perks::FiringLine,
            2888557110 => Perks::FullCourt,
            2782457288 => Perks::KillingTally,
            557221067 => Perks::KillingTally,
            3523296417 => Perks::Demolitionist,
            2458213969 => Perks::MultikillClip,
            4082225868 => Perks::Swashbuckler,

            //season 7 | year 2
            205890336 => Perks::UnderDog,
            3194351027 => Perks::ExplosiveLight,
            699525795 => Perks::EyeOfTheStorm,
            2866798147 => Perks::NoDistractions,

            //season 8 | year 3
            //TODO

            //season 9 | year 3
            2284787283 => Perks::ClownCartridge,
            3511092054 => Perks::ElementalCapacitor,
            1546637391 => Perks::Vorpal,

            //season 10 | year 3
            //bad season lmao

            //season 11 | year 3
            2450788523 => Perks::KillingWind,

            //season 12 | year 4
            25606670 => Perks::DualLoader,
            4049631843 => Perks::OneForAll,
            469285294 => Perks::Recombination,
            1523832109 => Perks::Reconstruction,
            3436462433 => Perks::Surplus,
            3967134106 => Perks::Surplus, // travelers chosen

            //season 13 | year 4
            951095735 => Perks::ImpulseAmplifier,
            4104185692 => Perks::Frenzy,
            3927722942 => Perks::LastingImpression,

            //season 14 | year 4
            1799762209 => Perks::Cornered,
            11612903 => Perks::AdrenalineJunkie,
            3418782618 => Perks::RewindRounds,
            1570042021 => Perks::HeatingUp,
            3824105627 => Perks::FireFly,
            960810156 => Perks::DangerZone,
            2946784966 => Perks::TunnelVision,

            //season 15 | year 4
            1195158366 => Perks::Encore,
            2621346526 => Perks::Ensemble,
            615063267 => Perks::Ensemble, //V-wing
            2610012052 => Perks::GoldenTricorn,
            438098033 => Perks::Harmony,
            1428297954 => Perks::PerpetualMotion,
            3673922083 => Perks::Adagio,

            //season 16 | year 5
            3078487919 => Perks::BaitAndSwitch,
            671806388 => Perks::CompulsiveReloader,
            2896038713 => Perks::FocusedFury,
            2978966579 => Perks::ChillClip,
            2172504645 => Perks::SleightOfHand,
            1583705720 => Perks::StatsForAll,
            509074078 => Perks::SteadyHands,
            2652708987 => Perks::SuccesfulWarmup,
            2224838837 => Perks::UnstoppableForce,

            //season 17 | year 5
            2451262963 => Perks::FragileFocus,
            744594675 => Perks::WellRounded,

            //season 18 | year 5
            1365187766 => Perks::GutShot,
            691659142 => Perks::Pugilist,
            1821614984 => Perks::Slickdraw,
            1870851715 => Perks::OverUnder,

            //season 19 | year 5
            3751912585 => Perks::CascadePoint,
            1782407750 => Perks::CloseToMelee,
            2416023159 => Perks::OffhandStrike,
            2272927194 => Perks::PerfectFloat,
            2586829431 => Perks::ShotSwap,
            365154968 => Perks::TargetLock,

            //season 20 | year 6
            3619207468 => Perks::KeepAway,

            //subclass
            3066103999 => Perks::OnYourMark,
            3469412970 => Perks::Hedrons,
            4194622036 => Perks::FlowState,
            1727069361 => Perks::Frequency,
            83039194 => Perks::HeatRises,
            362132290 => Perks::Tempering,

            //kinetic exotic
            1301843770 => Perks::CranialSpike,
            970163821 => Perks::AgersCall,
            2121086290 => Perks::RatPack,
            4004944400 => Perks::StringofCurses,
            3602718766 => Perks::RocketTracers,
            213689231 => Perks::ParacausalShot,
            3556949035 => Perks::TimeSlip,
            2724693746 => Perks::ToM,
            4208418110 => Perks::CorruptionSpreads,
            1070100196 => Perks::HonedEdge,
            2206869417 => Perks::HakkeHeavyBurst,
            3668782036 => Perks::FateOfAllFools,
            1791592647 => Perks::ExplosiveShadow,
            2130042297 => Perks::TakenPredator,
            944506345 => Perks::SurosLegacy,
            1378047685 => Perks::SpinningUp,
            4012962526 => Perks::DualSpeedReceiver,
            2984682260 => Perks::FullStop,
            630329983 => Perks::RideTheBull,
            383825919 => Perks::HuntersTrance,

            //energy exotic
            2881100038 => Perks::LagragianSight,
            961505134 => Perks::IgnitionTrigger,
            2226793914 => Perks::GuidanceRing,
            3174300811 => Perks::LooksCanKill,
            656200654 => Perks::ConserveMomentum,
            407549716 => Perks::Broadside,
            2333607307 => Perks::Impetus,
            3117514172 => Perks::Stormbringer,
            1000724343 => Perks::PerfectFith,
            571267712 => Perks::PrismaticInferno,
            838219733 => Perks::CalculatedBalance,
            924149234 => Perks::Chimera,
            2003108620 => Perks::ThinTheHerd,
            2540536653 => Perks::RavenousBeast,
            431220296 => Perks::LordOfWolvesCatalyst,
            299272945 => Perks::ReleaseTheWolves,
            2656694271 => Perks::SwoopingTalons,
            2620589274 => Perks::Fundamentals, //others
            3081173348 => Perks::Fundamentals, //borealis

            //heavy exotic
            4148158229 => Perks::ReignHavoc,
            2812324400 => Perks::WormsHunger,
            1340292993 => Perks::WhisperCatalyst,
            3333994164 => Perks::DarkDescent,
            939227542 => Perks::TargetAquired,

            _ => Perks::Ignore,
        }
    }
}

pub fn get_perk_stats(
    _perks: Vec<Perk>,
    _input_data: CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> [HashMap<u32, i32>; 2] {
    let mut dynamic_stats: HashMap<u32, i32> = HashMap::new();
    let mut static_stats: HashMap<u32, i32> = HashMap::new();
    for perk in _perks {
        let perk_stats = dyanmic_perk_stats(&perk, &_input_data, _pvp, _cached_data);
        for (key, value) in perk_stats {
            let entry = dynamic_stats.entry(key).or_insert(0);
            *entry += value;
        }
        for (key, value) in perk.stat_buffs {
            let entry = static_stats.entry(key.clone()).or_insert(0);
            *entry += value;
        }
    }
    [dynamic_stats, static_stats]
}
fn dyanmic_perk_stats(
    _perk: &Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::Roadborn => sbr_roadborn(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OphidianAspect => {
            sbr_ophidian_aspects(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::DragonShadow => sbr_dragon_shadow(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Amplified => sbr_amplified(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Tempering => sbr_tempering(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OnYourMark => sbr_on_your_mark(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HeatRises => sbr_heat_rises(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Hedrons => sbr_hedrons(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ThreatDetector => {
            sbr_threat_detector(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FieldPrep => sbr_field_prep(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FirmlyPlanted => sbr_firmly_planted(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HipFireGrip => sbr_hip_fire_grip(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::MovingTarget => sbr_moving_target(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OpeningShot => sbr_opening_shot(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Outlaw => sbr_outlaw(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SlideShot => sbr_slide_shot(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SlideWays => sbr_slide_ways(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TapTheTrigger => sbr_tap_the_trigger(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AirAssault => sbr_air_assault(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FeedingFrenzy => sbr_feeding_frenzy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ElementalCapacitor => {
            sbr_elemental_capacitor(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::PulseMonitor => sbr_pulse_monitor(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KillingWind => sbr_killing_wind(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AdrenalineJunkie => {
            sbr_adrenaline_junkie(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Ensemble => sbr_ensemble(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Frenzy => sbr_frenzy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::PerpetualMotion => {
            sbr_perpetual_motion(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::PerfectFloat => sbr_perfect_float(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Pugilist => sbr_pugilist(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Encore => sbr_encore(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FragileFocus => sbr_fragile_focus(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OffhandStrike => sbr_offhand_strike(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::StatsForAll => sbr_stats_for_all(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SteadyHands => sbr_steady_hands(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::WellRounded => sbr_well_rounded(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Alacrity => sbr_alacrity(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Ambush => sbr_ambush(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FluidDynamics => sbr_fluid_dynamics(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::QuietMoment => sbr_quiet_moment(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BitterSpite => sbr_bitter_spite(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RightHook => sbr_right_hook(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BackupPlan => sbr_backup_plan(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::DangerZone => sbr_danger_zone(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SleightOfHand => sbr_sleight_of_hand(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Slickdraw => sbr_slickdraw(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Harmony => sbr_harmony(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::CompulsiveReloader => {
            sbr_compulsive_reloader(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::RapidHit => sbr_rapid_hit(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ExplosiveLight => {
            sbr_explosive_light(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::ReleaseTheWolves => {
            sbr_release_the_wolves(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Fundamentals => sbr_fundamentals(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ThinTheHerd => sbr_thin_the_herd(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Chimera => sbr_chimera(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::DualSpeedReceiver => {
            sbr_dual_speed_receiver(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Surplus => sbr_surplus(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::QuickDraw => sbr_quickdraw(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TexBalancedStock => {
            sbr_tex_balanced_stock(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::EyeOfTheStorm => {
            sbr_eye_of_the_storm(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::HeatingUp => sbr_heating_up(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ImpulseAmplifier => {
            sbr_impulse_amplifier(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::SurosSynergy => sbr_suros_synergy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TunnelVision => sbr_tunnel_vision(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ShotSwap => sbr_shot_swap(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::UnderDog => sbr_underdog(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::UnderPressure => sbr_under_pressure(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Adagio => sbr_adagio(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HuntersTrance => sbr_hunters_trance(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KeepAway => sbr_keep_away(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FieldTested => sbr_field_tested(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RallyBarricade => sbr_rally_barricade(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TomeOfDawn => sbr_tome_of_dawn(_input_data, val, enhanced, _pvp, _cached_data),
        _ => HashMap::new(),
    }
}

pub fn get_dmg_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut dmg_modifier = DamageModifierResponse::new();
    for perk in _perks {
        let tmp = get_perk_dmr(perk.clone(), _input_data, _pvp, _cached_data);
        dmg_modifier.impact_dmg_scale *= tmp.impact_dmg_scale;
        dmg_modifier.explosive_dmg_scale *= tmp.explosive_dmg_scale;
        dmg_modifier.crit_scale *= tmp.crit_scale;
    }
    dmg_modifier
}
fn get_perk_dmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::HighImpactReserves => {
            dmr_high_impact_reserves(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::BoxBreathing => dmr_box_breathing(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ExplosivePayload => {
            dmr_explosive_payload(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::TimedPayload => dmr_timed_payload(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ImpactCasing => dmr_impact_casing(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ExplosiveHead => dmr_explosive_head(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FiringLine => dmr_firing_line(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KillingTally => dmr_killing_tally(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ResevoirBurst => dmr_resevoir_burst(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Surrounded => dmr_surrounded(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::LastingImpression => {
            dmr_lasting_impressions(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Vorpal => dmr_vorpal(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Adagio => dmr_adagio(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AdrenalineJunkie => {
            dmr_adrenaline_junkie(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Frenzy => dmr_frenzy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FocusedFury => dmr_focused_fury(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::GutShot => dmr_gutshot_straight(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TargetLock => dmr_target_lock(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OverUnder => dmr_over_under(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::WormsHunger => dmr_worms_hunger(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::LagragianSight => {
            dmr_lagragian_sight(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::EmpowermentBuffs => {
            dmr_empowerment_buffs(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::WeakenDebuffs => dmr_weaken_debuffs(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BuiltIn => dmr_builtin(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BossSpec => dmr_boss_spec(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::MajorSpec => dmr_major_spec(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::MinorSpec => dmr_minor_spec(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BigOnesSpec => dmr_big_ones_spec(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TakenSpec => dmr_taken_spec(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Rampage => dmr_rampage(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ToM => dmr_tom(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KillClip => dmr_kill_clip(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BackupPlan => dmr_backup_plan(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::DisruptionBreak => {
            dmr_disruption_break(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FullCourt => dmr_full_court(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OneForAll => dmr_one_for_all(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::GoldenTricorn => dmr_golden_tricorn(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BaitAndSwitch => dmr_bait_and_switch(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Swashbuckler => dmr_swash_buckler(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Harmony => dmr_harmony(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AcceleratedCoils => {
            dmr_accelerated_coils(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::ChargetimeMW => {
            dmr_accelerated_coils(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FullChoke => dmr_full_choke(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::LiquidCoils => dmr_liquid_coils(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HakkeHeavyBurst => {
            dmr_hakke_heavy_burst(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::MultikillClip => dmr_multi_kill_clip(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SpikeGrenades => dmr_spike_grenades(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ExplosiveLight => {
            dmr_explosive_light(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::DisorientingGrenades => {
            dmr_disorienting_grenades(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::SwoopingTalons => {
            dmr_swooping_talons(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::IgnitionTrigger => {
            dmr_ignition_trigger(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::CalculatedBalance => {
            dmr_vex_catalyst(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::RavenousBeast => dmr_ravenous_beast(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ReleaseTheWolves => {
            dmr_release_the_wolves(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FirstGlance => dmr_first_glance(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FateOfAllFools => {
            dmr_fate_of_all_fools(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::HonedEdge => dmr_honed_edge(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TakenPredator => dmr_taken_predator(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::MarkovChain => dmr_markov_chain(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::StormAndStress => {
            dmr_storm_and_stress(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FullStop => dmr_full_stop(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ParacausalShot => {
            dmr_paracausal_shot(_input_data, val, enhanced, _pvp, _cached_data)
        }
        _ => DamageModifierResponse::new(),
    }
}

pub fn get_reload_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let mut reload_modifier = ReloadModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_rsmr(perk, _input_data, _pvp, _cached_data);
        reload_modifier.reload_stat_add += tmp.reload_stat_add;
        reload_modifier.reload_time_scale *= tmp.reload_time_scale;
    }
    reload_modifier
}
fn get_perk_rsmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::RapidFireFrame => rsmr_alloy_mag(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AlloyMag => rsmr_alloy_mag(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Roadborn => rsmr_roadborn(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OphidianAspect => {
            rsmr_ophidian_aspects(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::DragonShadow => rsmr_dragon_shadow(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Frequency => rsmr_frequency(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FlowState => rsmr_flow_state(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OnYourMark => rsmr_on_your_mark(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ThreatDetector => {
            rsmr_threat_detector(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FieldPrep => rsmr_field_prep(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FeedingFrenzy => rsmr_feeding_frenzy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RapidHit => rsmr_rapid_hit(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ElementalCapacitor => {
            rsmr_elemental_capacitor(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Ensemble => rsmr_ensemble(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Frenzy => rsmr_frenzy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ImpulseAmplifier => {
            rsmr_impulse_amplifier(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::PerpetualMotion => {
            rsmr_perpetual_motion(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::StatsForAll => rsmr_stats_for_all(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Alacrity => rsmr_alacrity(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FluidDynamics => rsmr_fluid_dynamics(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::QuietMoment => rsmr_quiet_moment(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BitterSpite => rsmr_bitter_spite(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::LoaderMod => rsmr_loader_mods(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Outlaw => rsmr_outlaw(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FireFly => rsmr_fire_fly(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::CompulsiveReloader => {
            rsmr_compulsive_reloader(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::SleightOfHand => {
            rsmr_sleight_of_hand(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::ReleaseTheWolves => {
            rsmr_release_the_wolves(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Fundamentals => rsmr_fundamentals(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ThinTheHerd => rsmr_thin_the_herd(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Surplus => rsmr_surplus(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TexBalancedStock => {
            rsmr_tex_balanced_stock(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::UnderDog => rsmr_underdog(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HuntersTrance => rsmr_hunters_trance(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KeepAway => rsmr_keep_away(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FieldTested => rsmr_field_tested(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RallyBarricade => rsmr_rally_barricade(_input_data, val, enhanced, _pvp, _cached_data),
        _ => ReloadModifierResponse::default(),
    }
}

pub fn get_firing_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    let mut firing_modifier = FiringModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_fmr(perk, _input_data, _pvp, _cached_data);
        firing_modifier.burst_delay_scale *= tmp.burst_delay_scale;
        firing_modifier.burst_delay_add += tmp.burst_delay_add;
        firing_modifier.inner_burst_scale *= tmp.inner_burst_scale;
        firing_modifier.burst_size_add += tmp.burst_size_add;
    }
    firing_modifier
}
fn get_perk_fmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::Roadborn => fmr_roadborn(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Desperado => fmr_desperado(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ArchersTempo => fmr_archers_tempo(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Adagio => fmr_adagio(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Cornered => fmr_cornered(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::CascadePoint => fmr_cascade_point(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ReignHavoc => fmr_reign_havoc(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BackupPlan => fmr_backup_plan(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AcceleratedCoils => {
            fmr_accelerated_coils(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::ChargetimeMW => {
            fmr_accelerated_coils(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::LiquidCoils => fmr_liquid_coils(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HakkeHeavyBurst => {
            fmr_hakke_heavy_burst(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::AdeptChargeTime => {
            fmr_accelerated_coils(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::RavenousBeast => fmr_ravenous_beast(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ReleaseTheWolves => {
            fmr_release_the_wolves(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::AssaultMag => fmr_assault_mag(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FullAutoTrigger => {
            fmr_full_auto_trigger(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::RatPack => fmr_rat_pack(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SpinningUp => fmr_spinning_up(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RideTheBull => fmr_ride_the_bull(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FasterStringT1 => {
            fmr_faster_string_t1(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FasterStringT2 => {
            fmr_faster_string_t2(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::SlowerStringT1 => {
            fmr_slower_string_t1(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::SlowerStringT2 => {
            fmr_slower_string_t2(_input_data, val, enhanced, _pvp, _cached_data)
        }
        _ => FiringModifierResponse::default(),
    }
}

pub fn get_handling_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let mut handling_modifier = HandlingModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_hmr(perk, _input_data, _pvp, _cached_data);
        handling_modifier.handling_stat_add += tmp.handling_stat_add;
        handling_modifier.handling_swap_scale *= tmp.handling_swap_scale;
        handling_modifier.handling_ads_scale *= tmp.handling_ads_scale;
    }
    handling_modifier
}
fn get_perk_hmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::KillingWind => hmr_killing_wind(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SwapMag => hmr_swap_mag(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::QuickAccessSling => hmr_swap_mag(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OphidianAspect => {
            hmr_ophidian_aspects(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::DragonShadow => hmr_dragon_shadow(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Amplified => hmr_amplified(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OnYourMark => hmr_on_your_mark(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ThreatDetector => {
            hmr_threat_detector(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::FirmlyPlanted => hmr_firmly_planted(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Snapshot => hmr_snapshot(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ElementalCapacitor => {
            hmr_elemental_capacitor(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::AdrenalineJunkie => {
            hmr_adrenaline_junkie(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Ensemble => hmr_ensemble(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Frenzy => hmr_frenzy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::PerpetualMotion => {
            hmr_perpetual_motion(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Slickdraw => hmr_slickdraw(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::StatsForAll => hmr_stats_for_all(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SteadyHands => hmr_steady_hands(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::WellRounded => hmr_well_rounded(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HotSwap => hmr_hot_swap(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SearchParty => hmr_search_party(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::DexterityMod => hmr_dexterity_mods(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TargetingMod => hmr_targeting_mods(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BackupPlan => hmr_backup_plan(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SleightOfHand => hmr_sleight_of_hand(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Harmony => hmr_harmony(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SlideWays => hmr_slide_ways(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Fundamentals => hmr_fundamentals(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Chimera => hmr_chimera(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Surplus => hmr_surplus(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::QuickDraw => hmr_quickdraw(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TexBalancedStock => {
            hmr_tex_balanced_stock(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::PulseMonitor => hmr_pulse_monitor(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::EyeOfTheStorm => {
            hmr_eye_of_the_storm(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::SurosSynergy => hmr_suros_synergy(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::TunnelVision => hmr_tunnel_vision(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ShotSwap => hmr_shot_swap(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HuntersTrance => hmr_hunters_trance(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FieldTested => hmr_field_tested(_input_data, val, enhanced, _pvp, _cached_data),
        _ => HandlingModifierResponse::default(),
    }
}

pub fn get_magazine_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> MagazineModifierResponse {
    let mut magazine_modifier = MagazineModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_mmr(perk, _input_data, _pvp, _cached_data);
        magazine_modifier.magazine_stat_add += tmp.magazine_stat_add;
        magazine_modifier.magazine_add += tmp.magazine_add;
        magazine_modifier.magazine_scale *= tmp.magazine_scale;
    }
    magazine_modifier
}
fn get_perk_mmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> MagazineModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::AgersCall => mmr_agers_call(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::AmbitiousAssassin => {
            mmr_ambitious_assassin(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::OverFlow => mmr_overflow(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ClownCartridge => {
            mmr_clown_cartridge(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Reconstruction => mmr_reconstruction(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RunnethOver => mmr_runneth_over(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RatPack => mmr_rat_pack(_input_data, val, enhanced, _pvp, _cached_data),
        _ => MagazineModifierResponse::default(),
    }
}

pub fn get_reserve_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> InventoryModifierResponse {
    let mut reserve_modifier = InventoryModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_imr(perk, _input_data, _pvp, _cached_data);
        reserve_modifier.inv_stat_add += tmp.inv_stat_add;
        reserve_modifier.inv_add += tmp.inv_add;
        reserve_modifier.inv_scale *= tmp.inv_scale;
    }
    reserve_modifier
}
fn get_perk_imr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> InventoryModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::FieldPrep => imr_field_prep(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ReserveMod => imr_reserve_mods(_input_data, val, enhanced, _pvp, _cached_data),
        _ => InventoryModifierResponse::default(),
    }
}

pub fn get_range_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    let mut range_modifier = RangeModifierResponse::default();
    for perk in _perks {
        let tmp = get_perk_rmr(perk, _input_data, _pvp, _cached_data);
        range_modifier.range_stat_add += tmp.range_stat_add;
        range_modifier.range_all_scale *= tmp.range_all_scale;
        range_modifier.range_hip_scale *= tmp.range_hip_scale;
        range_modifier.range_zoom_scale *= tmp.range_zoom_scale;
    }
    range_modifier
}
fn get_perk_rmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::HipFireGrip => rmr_hip_fire_grip(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OpeningShot => rmr_opening_shot(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RangeFinder => rmr_range_finder(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::SlideShot => rmr_slide_shot(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KillingWind => rmr_killing_wind(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FragileFocus => rmr_fragile_focus(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::OffhandStrike => rmr_offhand_strike(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::StatsForAll => rmr_stats_for_all(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::WellRounded => rmr_well_rounded(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Alacrity => rmr_alacrity(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::KeepAway => rmr_keep_away(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RightHook => rmr_right_hook(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Encore => rmr_encore(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::Fundamentals => rmr_fundamentals(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::DualSpeedReceiver => {
            rmr_dual_speed_receiver(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::Adagio => rmr_adagio(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::HuntersTrance => rmr_hunters_trance(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FieldTested => rmr_field_tested(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::RallyBarricade => rmr_rally_barricade(_input_data, val, enhanced, _pvp, _cached_data),
        _ => RangeModifierResponse::default(),
    }
}

pub fn get_refund_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> Vec<RefundResponse> {
    let mut refund_modifier = vec![];
    for perk in _perks {
        let tmp = get_perk_refund(perk, _input_data, _pvp, _cached_data);
        if tmp.requirement > 0 {
            refund_modifier.push(tmp);
        }
    }
    refund_modifier
}
fn get_perk_refund(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RefundResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::TripleTap => rr_triple_tap(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::FourthTimesTheCharm => {
            rr_fourth_times(_input_data, val, enhanced, _pvp, _cached_data)
        }
        Perks::VeistStinger => rr_veist_stinger(_input_data, val, enhanced, _pvp, _cached_data),
        _ => RefundResponse::default(),
    }
}

pub fn get_extra_damage(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> Vec<ExtraDamageResponse> {
    let mut extra_damage = vec![];
    for perk in _perks {
        let tmp = get_perk_edr(perk, _input_data, _pvp, _cached_data);
        if tmp.additive_damage > 0.0 {
            extra_damage.push(tmp);
        }
    }
    extra_damage
}
fn get_perk_edr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ExtraDamageResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::ReignHavoc => edr_reign_havoc(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::ClusterBomb => edr_cluster_bomb(_input_data, val, enhanced, _pvp, _cached_data),
        Perks::BaitAndSwitch => edr_bait_and_switch(_input_data, val, enhanced, _pvp, _cached_data),
        _ => ExtraDamageResponse::default(),
    }
}

pub fn get_reload_overrides(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> Vec<ReloadOverrideResponse> {
    let mut reload_overrides = vec![];
    for perk in _perks {
        let tmp = get_perk_ror(perk, _input_data, _pvp, _cached_data);
        if tmp.valid {
            reload_overrides.push(tmp);
        }
    }
    reload_overrides
}
fn get_perk_ror(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadOverrideResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::Demolitionist => ror_demolitionist(_input_data, val, enhanced, _pvp, _cached_data),
        _ => ReloadOverrideResponse::invalid(),
    }
}

pub fn get_explosion_data(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> ExplosivePercentResponse {
    let mut highest_so_far = ExplosivePercentResponse::default();
    for perk in _perks {
        let tmp = get_perk_epr(perk, _input_data, _pvp);
        if tmp.percent > highest_so_far.percent {
            highest_so_far = tmp;
        }
    }
    highest_so_far
}

fn get_perk_epr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> ExplosivePercentResponse {
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::ExplosivePayload => {
            epr_explosive_payload(_input_data, val, enhanced, _pvp, &mut HashMap::new())
        }
        Perks::ExplosiveHead => {
            epr_explosive_head(_input_data, val, enhanced, _pvp, &mut HashMap::new())
        }
        Perks::TimedPayload => {
            epr_timed_payload(_input_data, val, enhanced, _pvp, &mut HashMap::new())
        }
        Perks::BuiltIn => epr_builtin(_input_data, val, enhanced, _pvp, &mut HashMap::new()),
        _ => ExplosivePercentResponse::default(),
    }
}

pub fn get_flinch_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FlinchModifierResponse {
    let mut tmp = FlinchModifierResponse::default();
    for perk in _perks{
        tmp.flinch_scale *= get_perk_flmr(perk, _input_data, _pvp).flinch_scale;
    }
    tmp
}

fn get_perk_flmr(
    _perk: Perk,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> FlinchModifierResponse{
    let perk_enum = _perk.hash.into();
    let val = _perk.value;
    let enhanced = _perk.enhanced;
    match perk_enum {
        Perks::SurosSynergy => flmr_suros_synergy(_input_data, val, enhanced, _pvp, &mut HashMap::new()),
        Perks::NoDistractions => flmr_no_distractions(_input_data, val, enhanced, _pvp, &mut HashMap::new()),
        Perks::UnflinchingMod => flmr_unflinching_mod(_input_data, val, enhanced, _pvp, &mut HashMap::new()),
        Perks::RallyBarricade => flmr_rally_barricade(_input_data, val, enhanced, _pvp, &mut HashMap::new()),
        Perks::TomeOfDawn => flmr_tome_of_dawn(_input_data, val, enhanced, _pvp, &mut HashMap::new()),
        //Perks::PerfectFloat => todo!(), //Perfect floats flinch resist value is unknown atm
        _ => FlinchModifierResponse::default(),
    }
}