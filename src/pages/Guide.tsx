import { createSignal, onMount } from 'solid-js'
import {
    Button,
    Table,
    TableBody,
    TableHead,
    TableRow,
    TableCell,
    Box,
} from '@suid/material'
import { Link } from '@solidjs/router'
import { invoke } from '@tauri-apps/api'

type GameResultParams = {
    id: number
    winner: string
    loser: string
    score: string
    held_at: string
}

const Guide = () => {
    const [gameResults, setGameResults] = createSignal<GameResultParams[]>([])

    onMount(async () => {
        setGameResults(await invoke('fetch_games'))
    })

    return (
        <>
            <Table>
                <TableHead>
                    <TableRow>
                        <TableCell>Game ID</TableCell>
                        <TableCell>Winner</TableCell>
                        <TableCell>Loser</TableCell>
                        <TableCell>Score</TableCell>
                        <TableCell>Date</TableCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    {gameResults().map((result) => (
                        <TableRow>
                            <TableCell>{result.id}</TableCell>

                            <TableCell>{result.winner}</TableCell>
                            <TableCell>{result.loser}</TableCell>
                            <TableCell>{result.score}</TableCell>
                            <TableCell>{result.held_at}</TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
            <Box textAlign="center">
                <Button variant="contained" as={Link} href="/gamemaker">
                    ゲームメイク
                </Button>
            </Box>
        </>
    )
}

export default Guide
