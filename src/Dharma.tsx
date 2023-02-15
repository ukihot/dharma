import { createSignal } from 'solid-js'
import { invoke } from '@tauri-apps/api/tauri'
import { Button } from '@suid/material'
import './Dharma.css'
import Scorer from './components/Scorer'
import OpsBoards from './components/OpsBoards'

function Dharma() {
    const [greetMsg, setGreetMsg] = createSignal('')
    const [name, setName] = createSignal('')

    const [hogeMsg, setHogeMsg] = createSignal('')

    async function greet() {
        setGreetMsg(await invoke('greet', { name: name() }))
    }

    async function raid_success() {
        setHogeMsg(await invoke('raid_success'))
    }

    return (
        <div class="container">
            <Scorer />
            <div class="row">
                <OpsBoards />
                <OpsBoards />
            </div>
        </div>
    )
}

export default Dharma
