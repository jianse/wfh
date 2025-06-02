<script setup lang="ts">
// import Button from "primevue/button"
import Menubar from "primevue/menubar"
import type { MenuItem } from "primevue/menuitem";

const menu: MenuItem[] = [
  {
    label: "战甲",
    route: { name: 'warframes' }
  },
  {
    label: "资源",
    route: { name: 'resources' }
  },
  {
    label: "配方",
    route: { name: 'recipes' }
  },
  {
    label: "振幅晶体",
    route: { name: 'mods' }
  },
  {
    label: "武器",
    route: { name: 'weapons' }
  },
  {
    label: "遗物",
    route: { name: 'relics' }
  },
  {
    label: "清单",
    route: { name: 'manifest' }
  },
  {
    label: "赋能",
    route: { name: 'arcane' }
  },
  {
    label: "关于",
    route: { name: 'about' }
  }
]
</script>

<template>
  <Menubar :model="menu">
    <template #item="{ item, props, hasSubmenu }">
      <router-link v-if="item.route" v-slot="{ href, navigate }" :to="item.route" custom>
        <a v-ripple :href="href" v-bind="props.action" @click="navigate">
          <!-- <span :class="item.icon" /> -->
          <span>{{ item.label }}</span>
        </a>
      </router-link>
      <a v-else v-ripple :href="item.url" :target="item.target" v-bind="props.action">
        <!-- <span :class="item.icon" /> -->
        <span>{{ item.label }}</span>
        <span v-if="hasSubmenu" class="pi pi-fw pi-angle-down" />
      </a>
    </template>
  </Menubar>
  <div style="height:calc(100vh - 58px);overflow: auto;">
    <Suspense>
      <RouterView> </RouterView>
    </Suspense>
  </div>
</template>

<style scoped>
.card {
  width: 500px;
  height: 200px;
  flex: initial;
  border: 1px solid #ccc;
  border-radius: 4px;
  padding: 8px;
}

.sticky-header {
  position: sticky;
  top: 0;
  height: 5rem;
  background-color: #fff;
  border-bottom: 1px solid #333;
  display: flex;
  flex-direction: row;
  gap: 10px;
  padding: 10px;
}

.card img {
  object-fit: contain;
  height: 150px;
}
</style>
