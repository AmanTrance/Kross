import { StrictMode, useState } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.jsx'
import './index.css'
import Signin from './components/Signin.jsx'
import {BrowserRouter, Routes, Route} from 'react-router-dom'
import store from './store/store.js'
import { Provider } from 'react-redux' 

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <Provider store={store}>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Signin/>}/>
          <Route path="/app" element={<App/>}/>        
        </Routes>
      </BrowserRouter>
    </Provider>
  </StrictMode>,
)
