import './App.css'
import Profile from './components/Profile.jsx'
import Arena from './components/Arena.jsx'
import { useEffect, useState } from 'react';
import axios from 'axios';

function App() {
  const [query, setQuery] = useState(0);
  const [components, setComponents] = useState([]);

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

  return (
    <div id='app-div'>
      <Profile/>
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
