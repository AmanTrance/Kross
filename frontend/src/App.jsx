import { useState } from 'react'
import './App.css'
import { useNavigate } from 'react-router-dom'

function App() {
  const navigate = useNavigate()
  return (
    <div id ='app-div' 
    onClick={() => navigate('/')}>
    </div>
  )
}

export default App
