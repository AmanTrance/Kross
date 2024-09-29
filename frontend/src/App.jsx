import './App.css';
import Profile from './components/Profile.jsx';
import Arena from './components/Arena.jsx';
import { useEffect, useState } from 'react';
import axios from 'axios';
import { v4 as uuid } from 'uuid';

function App() {
  const [query, setQuery] = useState(5);
  const [components, setComponents] = useState([]);
  const [click, setClick] = useState(false);
  
  useEffect(() => {
    const getArenas = async () => {
      let messages = [];
      let ids = [];
      const response = await axios.get(`http://127.0.0.1:8000/api/getarena/${window.sessionStorage.getItem('id')}`, {params: {
        limit: query
      }});
      if(response.data?.data !== "Arena not filled") {
        for (let data of response.data.data){
          messages = [...messages, data.message];
          ids = [...ids, data.owner_id];
        }
      }
      setComponents([]);
      for(let i = 0; i < query; i++){
        setComponents((prev) => {
          return [...prev, <Arena key={uuid()} msg={messages.length > i ? messages[i] : "No More Arenas"} id={ids[i]}/>];
        });
        if(messages.length === i) {
          break;
        }
      }
   }

   getArenas();
  }, [query]);

  const load_more = () => {
    setQuery((prev) => prev + 5);
  }

  const handlePost = async () => {
    const text = document.getElementById('text-box').value;
    const response = await axios.post('http://127.0.0.1:8000/api/arenapost', {
      owner_id: `${window.sessionStorage.getItem('id')}`,
      message: text
    }, {headers: {
      'Content-Type': 'application/json'
    }});
    if (response.status === 201){
      document.getElementById('text-box').value = "";
      setClick(false);
      document.querySelector('.fa-solid.fa-plus').style.transform = 'rotate(0deg)';
      document.getElementById('post-arena').style.height = '0px';
      document.getElementById('post-arena').style.width = '0px';
      document.getElementById('text-box').style.display = 'none';
      document.getElementById('post-btn').style.display = 'none';
    }
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
          <i className="fa-solid fa-plus" id='plus'></i>
        </div>
          <div id='post-arena'>
          <textarea id='text-box'></textarea>
          <div id='btn-div'>
            <button id='post-btn' onClick={handlePost}>Post</button>
          </div>
        </div>
      </div>
      {components.map((component) => component)}
      <div id='infinite-query'>
        <button id='load' onClick={load_more}>More</button>
      </div>   
    </div>
  )
}

export default App
