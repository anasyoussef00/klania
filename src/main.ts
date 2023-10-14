import App from './App.vue';

import { createApp } from 'vue';
import { createPinia } from 'pinia';

import './styles.css';
import './tailwind.css';

import router from './router';

const pinia = createPinia();

createApp(App).use(router).use(pinia).mount('#app');
