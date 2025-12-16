import archwing from './Archwing.png'
import toxin from './ToxinSymbol.png'
import corrosive from './CorrosiveSymbol.png'
import fire from './HeatSymbol.png'
import freeze from './ColdSymbol.png'
import gas from './GasSymbol.png'
import impact from './ImpactSymbol.png'
import magnetic from './MagneticSymbol.png'
import puncture from './PunctureSymbol.png'
import slash from './SlashSymbol.png'
import viral from './ViralSymbol.png'
import radiation from './RadiationSymbol.png'
import electricity from './ElectricitySymbol.png'
import radiant from './VoidSymbol.png'
import mb00_d from './MouseButton00_d.png'
import explosion from './BlastSymbol.png'
import energy from './Energy_d.png'
import mw_d from './MouseWheelButton_d.png'
import shield from './HildrynEnergyShield.png'
import sentient from './Sentient_d.png'

// TODO: AFFINITY_SHARE
const map = new Map<string, string>([
  ['ARCHWING', archwing],
  ['DT_POISON_COLOR', toxin],
  ['DT_CORROSIVE_COLOR', corrosive],
  ['DT_FIRE_COLOR', fire],
  ['DT_FREEZE_COLOR', freeze],
  ['DT_GAS_COLOR', gas],
  ['DT_IMPACT_COLOR', impact],
  ['DT_MAGNETIC_COLOR', magnetic],
  ['DT_PUNCTURE_COLOR', puncture],
  ['DT_VIRAL_COLOR', viral],
  ['DT_SLASH_COLOR', slash],
  ['DT_RADIATION_COLOR', radiation],
  ['DT_ELECTRICITY_COLOR', electricity],
  ['DT_RADIANT_COLOR', radiant],
  ['PRE_ATTACK', mb00_d],
  ['DT_EXPLOSION_COLOR', explosion],
  ['ENERGY', energy],
  ['SECONDARY_FIRE', mw_d],
  ['SHIELD', shield],
  ['DT_SENTIENT', sentient],
  ['DT_MAGNETIC', magnetic],
])

export function findImageUrl(name: string): string {
  const url = map.get(name)
  if (url === undefined) {
    console.warn(`No image found for ${name}`)
    return ''
  }
  return url
}
