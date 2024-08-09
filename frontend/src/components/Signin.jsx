import React from 'react'
import './Signin.css'

function Signin() {
  return (
    <div id='form-box'>
        <div id='form'>
        <form method='post' id ='main-form'>
            <label htmlFor='name' >NAME</label><br/>
            <input type='text' placeholder='Enter Name' id='name'/><br/>
            <label htmlFor='age'>AGE</label><br/>
            <input type='text' placeholder='Enter Age' id='age'/><br/>
            <label htmlFor='email'>EMAIL</label><br/>
            <input type='text' placeholder='Enter Email' id='email'/><br/>
            <input type='submit' value={"SIGN IN"} id='save-btn'/>
        </form>
        </div>
    </div>
  )
}

export default Signin