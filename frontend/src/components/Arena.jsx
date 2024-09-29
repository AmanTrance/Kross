import { useEffect, useState } from 'react';
import './Arena.css';
import axios from 'axios';

function Arena(props) {
  const [name, setName] = useState(null);

  useEffect(() => {
    const getUserName = async () => {
      const response = await axios.get(`http://127.0.0.1:8000/api/userdata/${props?.id}`);
      setName(response.data.name);
    }

    getUserName();
  }, []);

  return (
    <div id='box'>
        <div id='arena-box'>
          {props.id !== null ? <div id='pfp_wrapper'>
            <div id='pfp' style={{
              backgroundImage: `url(http://127.0.0.1:8000/api/getimg/${props?.id})`
            }}></div>
            <span id='pfp_username'>{name}</span>
          </div>: null}
          {props.msg}
        </div>
    </div>
  )
}

export default Arena 