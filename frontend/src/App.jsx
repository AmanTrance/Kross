import { useState } from 'react'
import './App.css'
import Profile from './components/Profile.jsx'
import Arena from './components/Arena.jsx'


function App() {
  const colorlist = ["violet", "blueviolet", "purple", "pink", "magenta"];

  return (
    <div id='app-div'>
    <Profile/>
    {colorlist.map((color) =>{
      return <Arena color={color}/>
    } )}      
    </div>
  )
}

export default App
