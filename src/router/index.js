import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'Layout',
    component: () => import('@/components/layout/index.vue'),
    children: [
      {
        path: '/home',
        name: 'Home',
        meta: {
          keepAlive: true,
        },
        component: () => import('@/views/home/index.vue'),
      },
      {
        path: '/search',
        name: 'Search',
        meta: {
          keepAlive: true,
        },
        component: () => import('@/views/search/index.vue'),
      },
      {
        path: '/add-task',
        name: 'AddTask',
        meta: {
          keepAlive: false,
        },
        component: () => import('@/views/video/CodecConvert.vue'),
      },
    ],
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
