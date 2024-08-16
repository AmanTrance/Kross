import React, { useEffect } from 'react'
import './Arena.css'

function Arena(props) {

  return (
    <div id='box'>
        <div id='arena-box'>
          {props.msg}
        </div>
    </div>
  )
}

export default Arena 