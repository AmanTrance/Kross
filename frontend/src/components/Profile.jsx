import React, { useState } from 'react'
import './Profile.css'

function Profile() {
  const [icon, setIcon] = useState(null);
  const edit = (e) => {
    if (e.type === 'mouseover'){
      setIcon(<i className="fa-solid fa-pen" id='icon'></i>);
      document.getElementById('profile-pic').style.backgroundImage = 'linear-gradient(grey, black)';
    }
    else{
      setIcon(null);
      document.getElementById('profile-pic').style.backgroundImage = 'linear-gradient(cyan, lightblue)'
    }
  }

  return (
   <div id='profile-pic'> 
   <label htmlFor='upload' id='image'>
    <div id='label' onMouseOver={edit} onMouseOut={edit}>
    {icon}
    </div>
   </label>
   <input type='file' id='upload'/>       
   </div>
  )
}

export default Profile