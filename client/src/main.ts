import { createApp } from 'vue'
import { Quasar } from 'quasar'
import 'quasar/src/css/index.sass'
import '@quasar/extras/fontawesome-v6/fontawesome-v6.css'
import './style.scss'
import App from './App.vue'

const app = createApp(App)
app.use(Quasar, {})
app.mount('#app')
