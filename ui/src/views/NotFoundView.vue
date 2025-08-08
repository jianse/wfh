<template>
  <div class="flex flex-col items-center justify-center h-screen">
    <div class="text-5xl font-bold">404</div>
    <pv-button asChild v-slot="slotProps">
      <RouterLink :to="{ name: 'home' }" :class="slotProps.class">返回首页</RouterLink>
    </pv-button>
  </div>
</template>
