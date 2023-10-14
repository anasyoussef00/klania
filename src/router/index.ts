import { createRouter, createWebHistory, type RouterOptions } from 'vue-router';
import routes from './routes';

const options = {
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
} satisfies RouterOptions;

const router = createRouter(options);

export default router;
