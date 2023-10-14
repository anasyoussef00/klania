import type { RouteRecordRaw } from 'vue-router';
import HomeView from '@views/HomeView.vue';

export default [
  {
    path: '/',
    name: 'home',
    component: HomeView,
  },
] satisfies readonly RouteRecordRaw[];
