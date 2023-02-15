import { invoke } from '@tauri-apps/api/tauri'
import Scorer from './pages/Scorer'
import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
import { Button } from '@suid/material'
import { emit, listen } from '@tauri-apps/api/event'
import { Route, Routes } from '@solidjs/router'
import OpsBoard from './pages/OpsBoard'

const unlisten = await listen('click', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
})

function Dharma() {
    return (
        <Routes>
            <Route path="/" component={OpsBoard} />
            <Route path="/scorer" component={Scorer} />
        </Routes>
    )
}

export default Dharma
