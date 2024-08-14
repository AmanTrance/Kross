import React, { useEffect } from 'react'
import './Arena.css'

function Arena(props) {

  return (
    <div id='box' style={{background: props.color}}>
        <textarea id='arena-box'>

        </textarea>
    </div>
  )
}

export default Arena 