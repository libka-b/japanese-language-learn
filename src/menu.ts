import { invoke } from '@tauri-apps/api/core'
import { configManager, getMainDivElement } from './main'
import { showStats } from './stats'
import { createGroupMenu, createLessonMenu } from './lesson_menu'

export function createMenu() {
    const html = `
    <div class="menu">
        <button id="lessons">Go to lessons</button>
        <button id="view-stats">View Stats</button>
        <button id="quit">Quit</button>
    </div>
    `

    getMainDivElement().innerHTML = html

    document.getElementById('view-stats')!.onclick = async () => {
        await createGroupMenu(configManager, showStats)
    }

    document.getElementById('lessons')!.onclick = async () => {
        await createGroupMenu(configManager, createLessonMenu)
    }

    document.getElementById('quit')!.onclick = async () => {
        await quit()
    }
}

async function quit() {
    await invoke('exit_app')
}
