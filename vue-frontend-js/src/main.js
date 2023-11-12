import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'
import router from './components/router/router'
// Create the Vue app
const app = createApp(App)

// Use the router
app.use(router)

// Mount the app to the DOM
app.mount('#app')

