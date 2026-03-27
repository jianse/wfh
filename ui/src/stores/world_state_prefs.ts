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

export interface WorldStatePrefs {
  enabledSections: WorldStateSection[]
  layout: 'grid' | 'list'
}

const defaultSections: WorldStateSection[] = ['cycles', 'fissures', 'alerts', 'invasions', 'sortie']

export const useWorldStatePrefsStore = defineStore(
  'worldStatePrefs',
  () => {
    const prefs = ref<WorldStatePrefs>({
      enabledSections: defaultSections,
      layout: 'grid',
    })

    const enabledSections = computed(() => prefs.value.enabledSections)
    const layout = computed(() => prefs.value.layout)

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
      const allSections: WorldStateSection[] = [
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
      ]
      prefs.value.enabledSections = [...allSections]
    }

    function deselectAll() {
      prefs.value.enabledSections = []
    }

    function resetToDefault() {
      prefs.value.enabledSections = [...defaultSections]
    }

    return {
      prefs,
      enabledSections,
      layout,
      toggleSection,
      setLayout,
      selectAll,
      deselectAll,
      resetToDefault,
    }
  },
  { persist: true },
)
