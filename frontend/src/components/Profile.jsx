import React, { useState, useEffect } from 'react'
import './Profile.css'
import { useSelector } from 'react-redux';
import axios from 'axios';

function Profile() {
  const [id, setId] = useState(useSelector((state) => state.id.id));
  const [name, setName] = useState('Aman'); 
  const [icon, setIcon] = useState(null);

  useEffect(() => {
    let func = async () => {
      if(id !== null){
        window.sessionStorage.setItem("id", id);
        const response = await fetch(`http://127.0.0.1:8000/api/getimg/${id}`);
        if(response.status !== 404){
          document.getElementById('profile-pic').style.backgroundImage = `url('http://127.0.0.1:8000/api/getimg/${id}')`;
        }
      }
      else{
        const temp = window.sessionStorage.getItem("id");
        if(temp === null){
          alert("You should sign in");
        }
        else{
          const response = await fetch(`http://127.0.0.1:8000/api/getimg/${temp}`);
          if(response.status !== 404){
            document.getElementById('profile-pic').style.backgroundImage = `url('http://127.0.0.1:8000/api/getimg/${temp}')`;
          }
        }
      }
    };
    func();
    
  }, [id])

  const edit = (e) => {
    if (e.type === 'mouseover'){
      setIcon(<i className="fa-solid fa-pen" id='icon'></i>);
    }
    else{
      setIcon(null);
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