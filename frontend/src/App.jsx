import './App.css'
import Profile from './components/Profile.jsx'
import Arena from './components/Arena.jsx'
import { useEffect, useState } from 'react';
import axios from 'axios';

function App() {
  const [query, setQuery] = useState(0);
  const [components, setComponents] = useState([]);
  const [click, setClick] = useState(false);
  useEffect(() => {
    if(query === 0) {
      setQuery(5);
      return;
    }; 
    const getArenas = async () => {
      let messages = [];
      const response = await axios.get('http://127.0.0.1:8000/api/getarena/100');
      messages = [...messages, response.data.data[0].message];
      for(let i = 0; i < 5; i++){
        setComponents((prev) => {
          return [...prev, <Arena msg={messages[0]}/>];
        });
      }
   }

   getArenas();
  }, [query]);

  const load_more = () => {
    setQuery((prev) => prev + 5);
  }

  function handleClick(e) {
    if(click === false) {
      setClick(true);
      document.querySelector('.fa-solid.fa-plus').style.transform = 'rotate(45deg)';
      document.getElementById('text-box').style.display = 'initial';
      document.getElementById('post-btn').style.display = 'initial';
      document.getElementById('post-arena').style.height = '600px';
      document.getElementById('post-arena').style.width = '400px';
    }else {
      setClick(false);
      document.querySelector('.fa-solid.fa-plus').style.transform = 'rotate(0deg)';
      document.getElementById('post-arena').style.height = '0px';
      document.getElementById('post-arena').style.width = '0px';
      document.getElementById('text-box').style.display = 'none';
      document.getElementById('post-btn').style.display = 'none';
    }
  }

  return (
    <div id='app-div'>
      <div className='profile-user'>
        <Profile/>
        <div id='add-arena' onClick={handleClick}>
          <i class="fa-solid fa-plus" id='plus'></i>
        </div>
          <div id='post-arena'>
          <textarea id='text-box'></textarea>
          <div id='btn-div'>
            <button id='post-btn'>Post</button>
          </div>
        </div>
      </div>
      {components.map((component) => {
        return component;
      })}
      <div id='infinite-query'>
        <button id='load' onClick={load_more}>More...</button>
      </div>   
    </div>
  )
}

export default App
