<template>
  <div class="flex flex-col items-center justify-center h-screen gap-4">
    <div class="text-5xl font-bold">404</div>
    <RouterLink :to="{ name: 'home' }" class="btn btn-secondary">返回首页</RouterLink>
  </div>
</template>
