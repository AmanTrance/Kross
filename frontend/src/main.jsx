import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.jsx'
import './index.css'
import Signin from './components/Signin.jsx'
import {BrowserRouter, Routes, Route} from 'react-router-dom'
import Error from './components/Error.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Signin/>}/>
        <Route path='error' element={<Error/>}/>
        <Route path="/app" element={<App/>}/>        
      </Routes>
    </BrowserRouter>
  </StrictMode>,
)
