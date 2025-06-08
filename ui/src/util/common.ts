export type ManifestItem = {
  uniqueName: string
  textureLocation: string
}

export function imageUrl(uniqueName: string, mfItems: ManifestItem[]): string {
  const textureLocation = mfItems.find((it) => it.uniqueName === uniqueName)?.textureLocation

  return textureLocation ? `https://content.warframe.com/PublicExport${textureLocation}` : ''
}

import type { ToastMessageOptions } from 'primevue/toast'
// export warframe

import ToastEventBus from 'primevue/toasteventbus'

export function useCommonToast() {
  const ToastService = {
    add: (message: ToastMessageOptions) => {
      ToastEventBus.emit('add', message)
    },
    remove: (message: ToastMessageOptions) => {
      ToastEventBus.emit('remove', message)
    },
    removeGroup: (group: string) => {
      ToastEventBus.emit('remove-group', group)
    },
    removeAllGroups: () => {
      ToastEventBus.emit('remove-all-groups')
    },
  }
  return ToastService
}

import DynamicDialogEventBus from 'primevue/dynamicdialogeventbus'
import type { DialogServiceMethods } from 'primevue'
import { markRaw } from 'vue'

export function useCommonDialog() {
  const DialogService: DialogServiceMethods = {
    open: (content, options) => {
      const instance = {
        content: content && markRaw(content),
        options: options || {},
        data: options && options.data,
        close: (params: object) => {
          DynamicDialogEventBus.emit('close', { instance, params })
        },
      }

      DynamicDialogEventBus.emit('open', { instance })

      return instance
    },
  }
  return DialogService
}
