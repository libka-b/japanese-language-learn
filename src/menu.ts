import { invoke } from '@tauri-apps/api/core'
import { configManager, getMainDivElement } from './main'
import { showStats } from './stats'
import { createGroupMenu, createLessonMenu } from './lesson_menu'

export function createMenu(): void {
    const html = `
    <div class="menu">
        <button id="lessons">Go to lessons</button>
        <button id="view-stats">View Stats</button>
        <button id="quit">Quit</button>
    </div>
    `

    getMainDivElement().innerHTML = html

    document.getElementById('view-stats')!.onclick =
        async (): Promise<void> => {
            await createGroupMenu(configManager, showStats)
        }

    document.getElementById('lessons')!.onclick = async (): Promise<void> => {
        await createGroupMenu(configManager, createLessonMenu)
    }

    document.getElementById('quit')!.onclick = async (): Promise<void> => {
        await quit()
    }
}

async function quit(): Promise<void> {
    await invoke('exit_app')
}
