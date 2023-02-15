import { Link } from '@solidjs/router'
import { Button } from '@suid/material'

const Start = () => {
    return (
        <div>
            <h1>Welcome to Dharma</h1>
            <Button as={Link} href="/dashboard">
                Go to Dashboard
            </Button>
            <Button as={Link} href="/settings">
                Go to Settings
            </Button>
        </div>
    )
}

export default Start
