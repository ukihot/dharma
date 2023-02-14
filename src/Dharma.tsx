import { createSignal } from 'solid-js'
import { invoke } from '@tauri-apps/api/tauri'
import { Button } from '@suid/material'
import './Dharma.css'
import Scorer from './components/Scorer'

function Dharma() {
    const [greetMsg, setGreetMsg] = createSignal('')
    const [name, setName] = createSignal('')

    const [hogeMsg, setHogeMsg] = createSignal('')

    async function greet() {
        setGreetMsg(await invoke('greet', { name: name() }))
    }

    async function hoge() {
        setHogeMsg(await invoke('hoge', { name: name() }))
    }

    return (
        <div class="container">
            <Scorer/>
            <p>
                {greetMsg}
                {hogeMsg}
            </p>
        </div>
    )
}

export default Dharma
