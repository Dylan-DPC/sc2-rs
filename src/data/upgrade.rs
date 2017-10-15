
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum Upgrade {
    Invalid,
    CarrierLaunchSpeedUpgrade,
    GlialReconstitution,
    TunnelingClaws,
    ChitinousPlating,
    HisecAutoTracking,
    TerranBuildingArmor,
    TerranInfantryWeaponsLevel1,
    TerranInfantryWeaponsLevel2,
    TerranInfantryWeaponsLevel3,
    NeoSteelFrame,
    TerranInfantryArmorsLevel1,
    TerranInfantryArmorsLevel2,
    TerranInfantryArmorsLevel3,
    Stimpack,
    ShieldWall,
    PunisherGrenades,
    HighCapacityBarrels,
    BansheeCloak,
    RavenCorvidReactor,
    PersonalCloaking,
    TerranVehicleWeaponsLevel1,
    TerranVehicleWeaponsLevel2,
    TerranVehicleWeaponsLevel3,
    TerranShipWeaponsLevel1,
    TerranShipWeaponsLevel2,
    TerranShipWeaponsLevel3,
    ProtossGroundWeaponsLevel1,
    ProtossGroundWeaponsLevel2,
    ProtossGroundWeaponsLevel3,
    ProtossGroundArmorsLevel1,
    ProtossGroundArmorsLevel2,
    ProtossGroundArmorsLevel3,
    ProtossShieldsLevel1,
    ProtossShieldsLevel2,
    ProtossShieldsLevel3,
    ObserverGravaticBooster,
    GravaticDrive,
    ExtendedThermalLance,
    PsiStormTech,
    ZergMeleeWeaponsLevel1,
    ZergMeleeWeaponsLevel2,
    ZergMeleeWeaponsLevel3,
    ZergGroundArmorsLevel1,
    ZergGroundArmorsLevel2,
    ZergGroundArmorsLevel3,
    ZergMissileWeaponsLevel1,
    ZergMissileWeaponsLevel2,
    ZergMissileWeaponsLevel3,
    OverlordSpeed,
    Burrow,
    ZerglingAttackSpeed,
    ZerglingMovementSpeed,
    ZergFlyerWeaponsLevel1,
    ZergFlyerWeaponsLevel2,
    ZergFlyerWeaponsLevel3,
    ZergFlyerArmorsLevel1,
    ZergFlyerArmorsLevel2,
    ZergFlyerArmorsLevel3,
    InfestorEnergyUpgrade,
    CentrificalHooks,
    BattleCruiserEnableSpecializations,
    ProtossAirWeaponsLevel1,
    ProtossAirWeaponsLevel2,
    ProtossAirWeaponsLevel3,
    ProtossAirArmorsLevel1,
    ProtossAirArmorsLevel2,
    ProtossAirArmorsLevel3,
    WarpGateResearch,
    Charge,
    BlinkTech,
    PhoenixRangeUpgrade,
    NeuralParasite,
    TerranVehicleAndShipArmorsLevel1,
    TerranVehicleAndShipArmorsLevel2,
    TerranVehicleAndShipArmorsLevel3,
    DrillClaws,
    AdeptPiercingAttack,
    MagFieldLaunchers,
    EvolveGroovedSpines,
    EvolveMuscularAugments,
    BansheeSpeed,
    RavenRecalibratedExplosives,
    MedivacIncreaseSpeedBoost,
    LiberatorAgRangeUpgrade,
    DarkTemplarBlinkUpgrade,
}

impl Upgrade {
    pub fn from_id(id: u32) -> Self {
        match id {
            _ => Upgrade::Invalid
        }
    }
}
