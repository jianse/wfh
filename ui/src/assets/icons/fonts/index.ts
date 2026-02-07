import archwing from './Archwing.png'
import toxin from './ToxinSymbol.png'
import corrosive from './CorrosiveSymbol.png'
import fire from './HeatSymbol.png'
import freeze from './ColdSymbol.png'
import gas from './GasSymbol.png'
import impact from './ImpactSymbol.png'
import magnetic from './MagneticSymbol.png'
import punctureColor from './PunctureSymbol.png'
import slash from './SlashSymbol.png'
import viral from './ViralSymbol.png'
import radiationColor from './RadiationSymbol.png'
import electricityColor from './ElectricitySymbol.png'
import radiant from './VoidSymbol.png'
import mb00_d from './MouseButton00_d.png'
import explosion_color from './BlastSymbol.png'
import energy from './Energy_d.png'
import mw_d from './MouseWheelButton_d.png'
import shield from './HildrynEnergyShield.png'
import sentient from './Sentient_d.png'
import shardRedSimple from './ArchonAmarIcon.png'
import shardBlueSimple from './ArchonBorealIcon.png'
import shardYellowSimple from './ArchonNiraIcon.png'
import radiation from './Radiation_d.png'
import puncture from './Puncture_d.png'
import electricity from './Electricity_d.png'
import explosion from './Blast_d.png'
import shardOrangeSimple from './ArchonCrystalOrangeIcon.png'

// TODO: AFFINITY_SHARE
const map = new Map<string, string>([
  // weapon types
  ['ARCHWING', archwing],

  // damage types colored
  ['DT_POISON_COLOR', toxin],
  ['DT_CORROSIVE_COLOR', corrosive],
  ['DT_FIRE_COLOR', fire],
  ['DT_FREEZE_COLOR', freeze],
  ['DT_GAS_COLOR', gas],
  ['DT_IMPACT_COLOR', impact],
  ['DT_MAGNETIC_COLOR', magnetic],
  ['DT_PUNCTURE_COLOR', punctureColor],
  ['DT_VIRAL_COLOR', viral],
  ['DT_SLASH_COLOR', slash],
  ['DT_RADIATION_COLOR', radiationColor],
  ['DT_ELECTRICITY_COLOR', electricityColor],
  ['DT_RADIANT_COLOR', radiant],
  ['DT_EXPLOSION_COLOR', explosion_color],

  // damage types without color
  ['DT_SENTIENT', sentient],
  ['DT_MAGNETIC', magnetic],
  ['DT_RADIATION', radiation],
  ['DT_PUNCTURE', puncture],
  ['DT_ELECTRICITY', electricity],
  ['DT_EXPLOSION', explosion],

  // base properties
  ['SHIELD', shield],
  ['ENERGY', energy],

  // buttons
  ['PRE_ATTACK', mb00_d],
  ['SECONDARY_FIRE', mw_d],

  // shards
  ['SHARD_RED_SIMPLE', shardRedSimple],
  ['SHARD_BLUE_SIMPLE', shardBlueSimple],
  ['SHARD_YELLOW_SIMPLE', shardYellowSimple],
  ['SHARD_ORANGE_SIMPLE', shardOrangeSimple],
])

export function findImageUrl(name: string): string {
  const url = map.get(name)
  if (url === undefined) {
    console.warn(`No image found for ${name}`)
    return ''
  }
  return url
}
