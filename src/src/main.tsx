import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import 'bootstrap/dist/css/bootstrap.css';
import './index.css'

ReactDOM.createRoot(document.getElementById('root') as HTMLDivElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)

/*
 * Color scheme setup
 */

// by default, auto color scheme
;(document.getElementById('root') as HTMLDivElement).classList.add('auto-scheme')

const rootClasses = (document.getElementById('root') as HTMLDivElement).classList.toString()
const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches

const lightBgColor = '#f1f5f9'
const darkBgColor = '#1e293b'
// const darkBgColor = '#404040'

if (rootClasses == 'light-scheme' || (rootClasses == 'auto-scheme' && !prefersDark)) {
  document.documentElement.style.background = lightBgColor
} else {
  document.documentElement.style.background = darkBgColor
}
