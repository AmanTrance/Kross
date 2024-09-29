import './Error.css';
import { useLocation, useNavigate } from 'react-router-dom';

function Error() {
    const location = useLocation();
    const navigate = useNavigate();

    const try_again = () => {
        if (location.state === null) {
            navigate('/error');
        } else {
            navigate(`${location.state?.path}`);
        }
    }
  return (
    <div className='main'>
        <div id='message-box'>
            Dear User! 
            <span>{location.state === null ? "No Error" : location.state.msg}</span>
            <button id='try-btn' onClick={try_again}>Try Again</button>
        </div>
    </div>
  )
}

export default Error