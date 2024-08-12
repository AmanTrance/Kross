import { createSlice } from "@reduxjs/toolkit"

const initialState = {
    id: null
}

const idSlice = createSlice({
    name: 'id',
    initialState,
    reducers:{
        updateId: (state, action) => {
            state.id = action.payload
        }
    }
})
export const { updateId } = idSlice.actions

export default idSlice.reducer