interface ArcaneExtInGame {
  uniqueName: string
  inGame: true
  slot: 'Warframe' | 'Operator' | 'Amp' | 'LongGun' | 'Pistol' | 'Melee' | 'Kitgun' | 'ZAW'
}

interface ArcaneExtUnknown {
  uniqueName: string
  inGame: false
  slot?: string
}

type ArcaneExt = ArcaneExtInGame | ArcaneExtUnknown

const ext: ArcaneExt[] = [
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/ArmourOnDamage',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/CorrosiveProcResist',
    inGame: false,
    slot: 'Unknown',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/CritChanceOnDamage',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/DamageResistanceStacksOverTime',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/ElectricityProcResist',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/FireProcResist',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/FreezeProcResist',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/GasProcResist',
    inGame: false,
    slot: 'Unknown',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/GolemArcaneShieldRegenOnDamage',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/HealthRegenOnDamage',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/ImpactProcResist',
    inGame: false,
    slot: 'Unknown',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/IncreaseMaxHealthOnHealthPickup',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/InstantShieldOnDamage',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/InvulnerabilityOnDeathOnMercyKill',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/MagneticProcResist',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/PoisonProcResist',
    inGame: false,
    slot: 'Unknown',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/PunctureProcResist',
    inGame: false,
    slot: 'Unknown',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/RadialViralAttackOnOverguardGain',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/RadiationProcResist',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/SlashProcResist',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/SpeedOnDamage',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Defensive/SpeedOnParry',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/AbilityHeatProcsGiveCritChance',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/AbilityPowerOnFreeze',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/AbilityStrengthForMaxHealth',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/AbilityStrengthOnCast',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/AbilityStrengthOnSummonAttack',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/AmmoEfficiencyOnPistolHeadshot',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/AmmoEfficiencyOnSliding',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/ArmorOnHeavyGunEquipped',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/ArmorOnMeleeKill',
    inGame: true,
    slot: 'Melee',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/BonusDamageNextShotForCurEnergy',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/ChannelKillEnergyRate',
    inGame: true,
    slot: 'ZAW',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/CompanionDamageOnResidualContact',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/CorrosiveMeleePerAbilityCast',
    inGame: true,
    slot: 'Melee',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/CritChannelingDamage',
    inGame: true,
    slot: 'ZAW',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/CritDamageForPunctureStatus',
    inGame: true,
    slot: 'Melee',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/DuplicateStatusOnKnock',
    inGame: true,
    slot: 'Melee',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/EyeInTheSky',
    inGame: true,
    slot: 'Kitgun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/FinisherLifesteal',
    inGame: true,
    slot: 'ZAW',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/GolemArcaneArmorOnFinisher',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/GolemArcaneMeleeDamageOnCrit',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/GolemArcanePistolDamageOnHeadshot',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/GolemArcanePistolSpeedOnCrit',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/GolemArcaneShotgunSpeedOnCrit',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/GolemArcaneSniperSpeedOnCrit',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/GroundSlamPull',
    inGame: true,
    slot: 'ZAW',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/HealCompanionOnSixMeleeKills',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/HealthRegenOnHeadshot',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/HeavyCritOnRegularMelee',
    inGame: true,
    slot: 'Melee',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/InvisibilityOnFinisher',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/LongGunDamageOnHeadshot',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/LongGunSpeedOnCrit',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/LongbowDamageOnHeadshot',
    inGame: true,
    slot: 'LongGun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/MaxEnergyForArmor',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/MeleeArcaneProjectileOnJump',
    inGame: true,
    slot: 'ZAW',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/MeleeArcaneShockwaveOnJump',
    inGame: true,
    slot: 'ZAW',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/MeleeDamageForActiveShields',
    inGame: true,
    slot: 'Melee',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/MeleeDamageOnCrit',
    inGame: false,
    slot: 'Unknown',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/MeleeDmgOnRifleKill',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/MeleeProcsSpread',
    inGame: true,
    slot: 'Melee',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/MeleeSpeedOnHit',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/MultishotForCombo',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName:
      '/Lotus/Upgrades/CosmeticEnhancers/Offensive/OperatorHeavyMeleeDamageOnTransference',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/OrbsOnResidualContact',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/OverguardSteal',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/PermaComboOnFinisher',
    inGame: true,
    slot: 'Melee',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/PowerStrengthOnKill',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/PrimaryCritDmgAndMultishotOnColdProc',
    inGame: true,
    slot: 'LongGun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/PrimaryDamageOnKill',
    inGame: true,
    slot: 'LongGun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/PrimaryDamageOnMeleeKill',
    inGame: true,
    slot: 'LongGun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/PrimaryDamageOnNoMelee',
    inGame: true,
    slot: 'LongGun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/PrimaryDamagePerAmmoOnReload',
    inGame: true,
    slot: 'LongGun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/PrimaryDmgOnMeleeKill',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/PrimaryEnergyRegenOnImpactProc',
    inGame: true,
    slot: 'LongGun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/PrimaryHeadshotCritMultOnHeadshot',
    inGame: true,
    slot: 'LongGun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/PrimaryJamEnemyWeaponsOnMagneticProc',
    inGame: true,
    slot: 'LongGun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/PullEnemiesOnMeleeKill',
    inGame: true,
    slot: 'Melee',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/ReplayLightCritHits',
    inGame: true,
    slot: 'Melee',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/SecondaryCritChancePerBuff',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/SecondaryCritOnHit',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/SecondaryDamageOnHeatProc',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/SecondaryDamageOnKill',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/SecondaryDamageOnMeleeKill',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/SecondaryDamageOnNoMelee',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/SecondaryExtraDamagePerFreezeStack',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName:
      '/Lotus/Upgrades/CosmeticEnhancers/Offensive/SecondaryReloadSpeedAndMultishotOnElectricProc',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/SeekerProjOnHeadshotKill',
    inGame: true,
    slot: 'Kitgun',
  },
  {
    uniqueName:
      '/Lotus/Upgrades/CosmeticEnhancers/Offensive/ShotgunMultishotAndReloadSpeedOnCloseKill',
    inGame: true,
    slot: 'LongGun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/StatusAndAmmoEfficiencyOnWeakpointHit',
    inGame: true,
    slot: 'LongGun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/StatusChannelingDamage',
    inGame: true,
    slot: 'ZAW',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/StatusTriggerRadialDamage',
    inGame: true,
    slot: 'ZAW',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Offensive/WeaponDamageOnResidualContact',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorAmps/AttackSpeedOnKill',
    inGame: true,
    slot: 'Amp',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorAmps/CriticalChanceOnHeadshot',
    inGame: true,
    slot: 'Amp',
  },
  {
    uniqueName:
      '/Lotus/Upgrades/CosmeticEnhancers/OperatorAmps/IncreasedCriticalDamageOnCriticalStrike',
    inGame: true,
    slot: 'Amp',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorAmps/IncreasedDamageOnStatusProc',
    inGame: true,
    slot: 'Amp',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorAmps/OperatorAmmoRegenOnKill',
    inGame: true,
    slot: 'Amp',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorAmps/StatusChanceOnHeadshot',
    inGame: true,
    slot: 'Amp',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorAmps/VoidToElectricDamage',
    inGame: true,
    slot: 'Amp',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorAmps/VoidToHeatDamage',
    inGame: true,
    slot: 'Amp',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorAmps/VoidToPunctureDamage',
    inGame: true,
    slot: 'Amp',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorAmps/VoidToViralDamage',
    inGame: true,
    slot: 'Amp',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/ArmourOnOperatorMode',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/DamageReductionOnVoidMode',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/HealOnTransferenceIn',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/HealOnTransferenceOut',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/HealOnVoidDash',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/HealOnVoidMode',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/HealthOnOperatorMode',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/HealthRegenAndArmorOnMeleeKill',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/HeatDamageOnDash',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/HeatResistOnBlast',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/HoverboardSpeedOnTransferenceIn',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/ImmunityFallDamageOnVoidDash',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/NoPenaltyOnDeath',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/ProcResistOnBlast',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/PullOnTransferenceIn',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/ReverseOnDash',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/RobotStunOnBlast',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/SpeedOnVoidDash',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/OperatorArmour/TetherMineOnDash',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName:
      '/Lotus/Upgrades/CosmeticEnhancers/Utility/AbilityRadiationProcsCreateUniversalOrbsOnKill',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName:
      '/Lotus/Upgrades/CosmeticEnhancers/Utility/AbilityStatusProcsGiveAbilityStrengthAndEfficiency',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName:
      '/Lotus/Upgrades/CosmeticEnhancers/Utility/AbilityStrengthAndCritDamageWhenInvisible',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/BulletToBattery',
    inGame: true,
    slot: 'Kitgun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/BurningCystOnKill',
    inGame: true,
    slot: 'Kitgun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/DamageReductionDuringRevive',
    inGame: false,
    slot: 'Unknown',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/EnergyEfficiencyOnHeadshot',
    inGame: true,
    slot: 'Kitgun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/ExtraProcOnProc',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/GolemArcaneAimGlideOnHeadshot',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/GolemArcaneBonusDamageOnWallLatch',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/GolemArcaneRadialEnergyOnEnergyPickup',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/HealAlliesOnEnergySpent',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/HealthWhileUsingChanneledAbilities',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/IceMistOnKill',
    inGame: true,
    slot: 'Kitgun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/LightningStrikeOnKill',
    inGame: true,
    slot: 'Kitgun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/LongGunDamageOnReload',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/NoCostCastChanceOnCast',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/PistolDamageOnReload',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/RadialHealOnHealthPickup',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/RadialKnockdownOnEnergyPickup',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/SlowerBleedOutOnPredeath',
    inGame: false,
    slot: 'Unknown',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Utility/ToxicBloodOnKill',
    inGame: true,
    slot: 'Kitgun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Zariman/AmpOnEnergyDepletedCritChance',
    inGame: true,
    slot: 'Amp',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Zariman/AmpOnVoidSlingAmpAmmoEfficiency',
    inGame: true,
    slot: 'Amp',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Zariman/EnergyOnGhostDissipate',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Zariman/OperatorOnDeathInvulnerability',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Zariman/OperatorOnEnergyDepletedRegenEnergy',
    inGame: true,
    slot: 'Operator',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Zariman/OperatorOnOperatorAbilityIncreaseDamage',
    inGame: true,
    slot: 'Amp',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Zariman/PrimaryOnAbilityReloadSpeed',
    inGame: true,
    slot: 'LongGun',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Zariman/SecondaryOnOvershieldCritChance',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Zariman/SecondaryOnRollCritChanceOnHeadshot',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Zariman/SecondaryOnStatusProcBonusDamage',
    inGame: true,
    slot: 'Pistol',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Zariman/WarframeOnOperatorAbilityPowerStrength',
    inGame: true,
    slot: 'Warframe',
  },
  {
    uniqueName: '/Lotus/Upgrades/CosmeticEnhancers/Zariman/WarframeOnShieldUptimePowerDuration',
    inGame: true,
    slot: 'Warframe',
  },
]

export default ext
