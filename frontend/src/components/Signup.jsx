import { useState } from 'react';
import './Signup.css';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';
import img1 from '../public/eye-close.png';
import img2 from '../public/eye-open.png';

function Signup() {
  const [hide, setHide] = useState(true);
  const navigate = useNavigate();

  const submitform = async (e) => {
    e.preventDefault();
    const form = document.getElementById('main-form');
    const formdata = new FormData(form);
    const apidata = Object.fromEntries(formdata);
    const response = await axios.post('http://127.0.0.1:8000/api/signup', apidata, { headers: {
      'Content-Type': 'application/json'
    }});
    if (typeof response.data.data === 'string') {
        navigate('/error', { state: {
            msg: 'Username or Email already exists',
            path: '/signup'
        }});
    } else {
        navigate('/');
    }
  }

  const handleLink = () => {
    navigate('/');
  }

  const visibilityHandle = () => {
    if (hide === true) {
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
            <input type='text' placeholder='Enter username' id='name' name='name' required/><br/>
            <label htmlFor='email'>EMAIL</label><br/>
            <input type='text' placeholder='Enter email' id='email' name='email' pattern='^[a-z]+@[a-z]+.com$' title='invalid email' required/><br/>
            <label htmlFor='password'>PASSWORD</label><br/>
            <div id='passcontainer'>
              <input type='password' placeholder='Enter password' id='password' name='password' required/><br/>
              <img src={hide ? img1 : img2} id='secure' onClick={visibilityHandle}></img>
            </div>
            <a id='linktoin' onClick={handleLink}>Already have an account?</a>
            <input type='submit' value="Sign up" id='save-btn'/>
        </form>
    </div>
  )
}

export default Signup