import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export type WorldStateSection =
  | 'cycles'
  | 'events'
  | 'alerts'
  | 'fissures'
  | 'steelPath'
  | 'dailyDeals'
  | 'archonHunt'
  | 'sentientOutposts'
  | 'sortie'
  | 'voidTrader'
  | 'vaultTrader'
  | 'nightwave'
  | 'invasions'
  | 'conquests'
  | 'hexCalendar'
  | 'endlessXp'
  | 'syndicateMissions'

export interface WorldStatePrefs {
  enabledSections: WorldStateSection[]
  layout: 'grid' | 'list'
  fissureTiers: string[]
  hexCalendarEvents: string[]
  conquests: Record<string, boolean>
  panelExpanded: boolean
  anchorExpanded: boolean
}

export const ALL_SECTIONS: WorldStateSection[] = [
  'cycles',
  'events',
  'alerts',
  'fissures',
  'steelPath',
  'dailyDeals',
  'archonHunt',
  'sentientOutposts',
  'sortie',
  'voidTrader',
  'vaultTrader',
  'nightwave',
  'invasions',
  'conquests',
  'hexCalendar',
  'endlessXp',
  'syndicateMissions',
]

const defaultSections: WorldStateSection[] = ['cycles', 'fissures', 'alerts', 'invasions', 'sortie']
const defaultFissureTiers = ['VoidT1', 'VoidT2', 'VoidT3', 'VoidT4', 'VoidT5', 'VoidT6']
const defaultHexCalendarEvents = ['CET_CHALLENGE', 'CET_REWARD', 'CET_BIRTHDAY', 'CET_UPGRADE']
const defaultConquests: Record<string, boolean> = {}
const defaultPanelExpanded = true

export const useWorldStatePrefsStore = defineStore(
  'worldStatePrefs',
  () => {
    const prefs = ref<WorldStatePrefs>({
      enabledSections: defaultSections,
      layout: 'grid',
      fissureTiers: defaultFissureTiers,
      hexCalendarEvents: defaultHexCalendarEvents,
      conquests: defaultConquests,
      panelExpanded: defaultPanelExpanded,
      anchorExpanded: true,
    })

    const enabledSections = computed(() => prefs.value.enabledSections)
    const layout = computed(() => prefs.value.layout)
    const fissureTiers = computed(() => prefs.value.fissureTiers)
    const hexCalendarEvents = computed(() => prefs.value.hexCalendarEvents)
    const conquests = computed(() => prefs.value.conquests)
    const panelExpanded = computed(() => prefs.value.panelExpanded)
    const anchorExpanded = computed(() => prefs.value.anchorExpanded)

    function toggleSection(section: WorldStateSection) {
      const index = prefs.value.enabledSections.indexOf(section)
      if (index > -1) {
        prefs.value.enabledSections.splice(index, 1)
      } else {
        prefs.value.enabledSections.push(section)
      }
    }

    function setLayout(layout: 'grid' | 'list') {
      prefs.value.layout = layout
    }

    function selectAll() {
      prefs.value.enabledSections = [...ALL_SECTIONS]
    }

    function moveSection(from: number, to: number) {
      const list = prefs.value.enabledSections
      if (from < 0 || to < 0 || from >= list.length || to >= list.length || from === to) return
      const [it] = list.splice(from, 1)
      list.splice(to, 0, it)
    }

    function setSections(sections: WorldStateSection[]) {
      prefs.value.enabledSections = [...sections]
    }

    function deselectAll() {
      prefs.value.enabledSections = []
    }

    function resetToDefault() {
      prefs.value.enabledSections = [...defaultSections]
    }

    function setFissureTiers(tiers: string[]) {
      prefs.value.fissureTiers = tiers
    }

    function toggleFissureTier(tier: string) {
      const index = prefs.value.fissureTiers.indexOf(tier)
      if (index > -1) {
        prefs.value.fissureTiers.splice(index, 1)
      } else {
        prefs.value.fissureTiers.push(tier)
      }
    }

    function setHexCalendarEvents(events: string[]) {
      prefs.value.hexCalendarEvents = events
    }

    function toggleHexCalendarEvent(eventType: string) {
      const index = prefs.value.hexCalendarEvents.indexOf(eventType)
      if (index > -1) {
        prefs.value.hexCalendarEvents.splice(index, 1)
      } else {
        prefs.value.hexCalendarEvents.push(eventType)
      }
    }

    function setConquestHard(type: string, hard: boolean) {
      prefs.value.conquests[type] = hard
    }

    function getConquestHard(type: string): boolean {
      return prefs.value.conquests[type] || false
    }

    function togglePanel() {
      prefs.value.panelExpanded = !prefs.value.panelExpanded
    }

    function setPanelExpanded(expanded: boolean) {
      prefs.value.panelExpanded = expanded
    }

    function toggleAnchor() {
      prefs.value.anchorExpanded = !prefs.value.anchorExpanded
    }

    return {
      prefs,
      enabledSections,
      layout,
      fissureTiers,
      hexCalendarEvents,
      conquests,
      panelExpanded,
      anchorExpanded,
      toggleSection,
      setLayout,
      selectAll,
      deselectAll,
      resetToDefault,
      moveSection,
      setSections,
      setFissureTiers,
      toggleFissureTier,
      setHexCalendarEvents,
      toggleHexCalendarEvent,
      setConquestHard,
      getConquestHard,
      togglePanel,
      setPanelExpanded,
      toggleAnchor,
    }
  },
  { persist: true },
)
