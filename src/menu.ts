import { invoke } from '@tauri-apps/api/core'
import { getNextHiragana } from './hiragana_lesson'
import { getMainDivElement } from './main'
import { showStats } from './stats'

export function createMenu() {
    const html = `
    <div class="menu">
        <button id="start-lesson">Start Lesson</button>
        <button id="view-stats">View Stats</button>
        <button id="quit">Quit</button>
    </div>
    `

    getMainDivElement().innerHTML = html

    document.getElementById('view-stats')!.onclick = async () => {
        await showStats()
    }

    document.getElementById('start-lesson')!.onclick = async () => {
        await invoke('set_counter', { stopAt: 50 })
        await getNextHiragana()
    }

    document.getElementById('quit')!.onclick = async () => {
        await quit()
    }
}

async function quit() {
    await invoke('exit_app')
}
