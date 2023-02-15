import Scorer from './pages/Scorer'
import { Route, Routes } from '@solidjs/router'
import Logger from './pages/Logger'
import Register from './pages/Register'
import Start from './pages/Start'
import GameMaker from './pages/GameMaker'

function Dharma() {
    return (
        <Routes>
            ///スタート画面 責務：スコア画面以外へのルータ
            <Route path="/" component={Start} />
            /// マスタ画面 責務：プレーヤー・チームの登録
            <Route path="/register" component={Register} />
            /// ゲームメイク 責務：レコーディング画面への準備
            <Route path="/gamemaker" component={GameMaker} />
            /// レコーディング画面 責務：レイドリザルトの記録入力
            <Route path="/logger" component={Logger} />
            /// スコア画面 責務：ゲーム状況の表示出力
            <Route path="/scorer" component={Scorer} />
        </Routes>
    )
}

export default Dharma
