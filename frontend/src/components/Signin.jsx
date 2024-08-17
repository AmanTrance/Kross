import React, { useState } from 'react'
import './Signin.css'
import axios from 'axios'
import { useNavigate } from 'react-router-dom';
import img1 from '../public/eye-close.png';
import img2 from '../public/eye-open.png';

function Signin() {
  const [hide, setHide] = useState(true);
  const navigate = useNavigate();
  const submitform = async (e) => {
    e.preventDefault();
    const form = document.getElementById('main-form');
    const formdata = new FormData(form);
    const apidata = Object.fromEntries(formdata);
    const data = await axios.post('http://127.0.0.1:8000/api/user', JSON.stringify(apidata), {headers:{
      'Content-Type': 'application/json'
    }});
    const id = data.data.id;
    if(id !== "Wrong Credentials") {
      window.sessionStorage.setItem("id", id)
      navigate('/app')
    }else{
      navigate('/error', {state:
        {
          msg:"Wrong Credentials",
          path: "/"
      }})
    }
  }

  const visibilityHandle = () => {
    if(hide === true) {
      setHide(false);
      document.getElementById('password').type = "text";
    }else {
      setHide(true);
      document.getElementById('password').type = "password";
    }
  }

  return (
    <div id='form-box'>
        <form id ='main-form' onSubmit={submitform}>
            <label htmlFor='name' >USERNAME</label><br/>
            <input type='text' placeholder='Enter Username' id='name' name='name'/><br/>
            <label htmlFor='email'>EMAIL</label><br/>
            <input type='text' placeholder='Enter Email' id='email' name='email'/><br/>
            <label htmlFor='password'>PASSWORD</label><br/>
            <div id='passcontainer'>
              <input type='password' placeholder='Enter Password' id='password' name='password'/><br/>
              <img src={hide ? img1 : img2} id='secure' onClick={visibilityHandle}></img>
            </div>
            <input type='submit' value={"SIGN IN"} id='save-btn'/>
        </form>
    </div>
  )
}

export default Signin