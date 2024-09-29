import { createRoot } from 'react-dom/client';
import App from './App.jsx';
import './index.css';
import Signin from './components/Signin.jsx';
import {BrowserRouter, Routes, Route} from 'react-router-dom';
import Error from './components/Error.jsx';
import Signup from './components/Signup.jsx';

createRoot(document.getElementById('root')).render(
  <BrowserRouter>
    <Routes>
      <Route path='/' element={<Signin/>}/>
      <Route path='/signup' element={<Signup/>}/>
      <Route path='error' element={<Error/>}/>
      <Route path="/app" element={<App/>}/>        
    </Routes>
  </BrowserRouter>
)
