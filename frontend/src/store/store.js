import {configureStore} from "@reduxjs/toolkit"
import idReducer from '../components/idSlice.js'


const store = configureStore({
    reducer: {
        id: idReducer
    }
})

export default store