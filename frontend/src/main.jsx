import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.jsx'
import './index.css'
import Signin from './components/Signin.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <Signin />
    <App />
  </StrictMode>,
)