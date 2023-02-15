import { Link } from '@solidjs/router'
import { Button } from '@suid/material'

const Start = () => {
    return (
        <div class="row">
            <h1>Welcome to Dharma</h1>
            <Button as={Link} href="/register">
                マスタ
            </Button>
            <Button as={Link} href="/gamemaker">
                ゲームメイク
            </Button>
            <Button as={Link} href="/logger">
                レコーディング
            </Button>
        </div>
    )
}

export default Start
