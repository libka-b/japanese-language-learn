import { invoke } from '@tauri-apps/api/core'
import { getMainDivElement } from './main'
import { showStats } from './stats'
import { createLessonMenu } from './lesson_menu'

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
        await createLessonMenu()
    }

    document.getElementById('quit')!.onclick = async () => {
        await quit()
    }
}

async function quit() {
    await invoke('exit_app')
}
