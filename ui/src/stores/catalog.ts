import { defineStore } from 'pinia'

/** 数据库视图的筛选 / 排序 / 搜索状态（跨视图与全局搜索共享）。 */
export const useCatalogStore = defineStore('catalog', {
  state: () => ({
    wfCat: 'all',
    wfQuery: '',
    wfSort: 'default',
    wpCat: 'all',
    wpQuery: '',
    wpSort: 'default',
    modCat: 'all',
    modQuery: '',
    arcaneQuery: '',
    relicQuery: '',
    resourceQuery: '',
    recipeQuery: '',
  }),
  actions: {
    setWarframeCat(cat: string) {
      this.wfCat = cat
    },
  },
})
