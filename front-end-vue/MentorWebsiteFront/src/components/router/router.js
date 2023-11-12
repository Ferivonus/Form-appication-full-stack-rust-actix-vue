import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'home',
    component: () => import('../Home.vue')
  },
  {
    path: '/NotFound',
    name: 'notFound',
    component: () => import('../NotFound.vue')
  },
  {
    path: '/hizmetler',
    name: 'hizmet',
    component: () => import('../Hizmet.vue')
  },
  {
    path: '/test',
    name: 'test',
    component: () => import('../Test.vue')
  },
  {
    path: '/vizyon',
    name: 'vizyon',
    component: () => import('../Vizyon.vue')
  },
  {
    path: '/paketler',
    name: 'paketler',
    component: () => import('../Paketler.vue')
  },
  {
    path: '/hizmetler/mentorlar',
    name: 'mentorlar',
    component: () => import('../HizmetlerAlt/Mentorlar.vue')
  },
  {
    path: '/hizmetler/ozelders',
    name: 'ozelders',
    component: () => import('../HizmetlerAlt/OzelDers.vue')
  },
  {
    path: '/hizmetler/psikologlar',
    name: 'psikologlar',
    component: () => import('../HizmetlerAlt/Psikologlar.vue')
  },
  {
    path: '/satinalma',
    name: 'satinalma',
    component: () => import('../satinalma.vue')
  },
  // Add more routes here...
]

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router
