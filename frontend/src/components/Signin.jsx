import React from 'react'
import './Signin.css'
import axios from 'axios'
import { useDispatch } from 'react-redux'
import { updateId } from './idSlice';
import { useNavigate } from 'react-router-dom';

function Signin() {
  const navigate = useNavigate();
  const dispatch = useDispatch();
  const submitform = async (e) => {
    e.preventDefault();
    const form = document.getElementById('main-form');
    const formdata = new FormData(form);
    const apidata = Object.fromEntries(formdata);
    const data = await axios.post('http://127.0.0.1:8000/api/user', JSON.stringify(apidata), {headers:{
      'Content-Type': 'application/json'
    }});
    const response = data.data.id;
    dispatch(updateId(response));
    navigate('/app');
  }

  return (
    <div id='form-box'>
        <form id ='main-form' onSubmit={submitform}>
            <label htmlFor='name' >NAME</label><br/>
            <input type='text' placeholder='Enter Name' id='name' name='name'/><br/>
            <label htmlFor='email'>EMAIL</label><br/>
            <input type='text' placeholder='Enter Email' id='email' name='email'/><br/>
            <label htmlFor='password'>PASSWORD</label><br/>
            <input type='text' placeholder='Enter Password' id='password' name='password'/><br/>
            <input type='submit' value={"SIGN IN"} id='save-btn'/>
        </form>
    </div>
  )
}

export default Signin