import React, { useState, useEffect } from 'react'
import './Profile.css'
import { useSelector } from 'react-redux';
import axios from 'axios';

function Profile() {
  const [id, setId] = useState(useSelector((state) => state.id.id));
  const [name, setName] = useState('Aman'); 
  const [icon, setIcon] = useState(null);

  if(id !== null){
    useEffect(() => {
      document.getElementById('profile-pic').style.backgroundImage = `url('http://127.0.0.1:8000/api/getimg/${id}')`;
    })
    
  }

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

  const uploadProfile = async (e) => {
    e.preventDefault();
    if(id === null) {
      alert("You are not Signed In");
      return;
    }else{
      alert("Only Jpeg/jpg Files of max size 2Mb");
      const response = await axios.post(`http://127.0.0.1:8000/api/image/${id}`, e.target.files[0], {headers:{
        'Content-type': 'image/jpeg'
      }});
    }
  }

  return (
    <div className='sandwich'>
      <div id='profile-pic'> 
        <label htmlFor='upload' id='image'>
          <div id='label' onMouseOver={edit} onMouseOut={edit}>
            {icon}
          </div>
        </label>
        <input type='file' id='upload' onChange={uploadProfile}/>       
      </div>
      <span id='span'>Welcome {name}!</span>
   </div>
  )
}

export default Profile