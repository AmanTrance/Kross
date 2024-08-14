import React, { useState, useEffect } from 'react'
import './Profile.css'
import axios from 'axios';
import { useNavigate } from 'react-router-dom'

function Profile() {
  const [id, setId] = useState(window.sessionStorage.getItem("id"));
  const [name, setName] = useState('User'); 
  const [icon, setIcon] = useState(null);
  const navigate = useNavigate();

  useEffect(() => {
    let func = async () => {
      if(id !== null){
        const user_details = await axios.get(`http://127.0.0.1:8000/api/userdata/${id}`);
        const user_details_response = await user_details.data.name;
        setName(user_details_response);
        const response = await axios.get(`http://127.0.0.1:8000/api/getimg/${id}`);
        if(response.status !== 404){
          document.getElementById('profile-pic').style.backgroundImage = `url('http://127.0.0.1:8000/api/getimg/${id}')`;
        }
      }else{
        navigate('/error', {state:{
          msg: "Sign in first !!",
          path: "/"
        }})
      }
    }
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
      navigate('/error', {state: {
        msg: "Sign in first!!",
        path: "/"
      }})
      return;
    }else{
      const response = await axios.post(`http://127.0.0.1:8000/api/image/${id}`, e.target.files[0], {headers:{
        'Content-Type': 'image/jpeg'
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
      <span id='span'>Welcome {name} !</span>
   </div>
  )
}

export default Profile