import { createApp } from 'vue'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

import App from './App.vue'
import router from './router/index'
import plugins from './plugins/index'
import { Icon } from '@iconify/vue'

const app = createApp(App)
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}
app.component('Icon', Icon)
app.use(router).use(plugins).mount('#app')
