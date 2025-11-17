import { invoke } from '@tauri-apps/api/core'
import { getNextHiragana } from './hiragana_lesson'

export function createMenu(): { html: string, setup: () => void } {
    const html = `
    <div>
        <button id="start-lesson">Start Lesson</button>
        <button id="quit">Quit</button>
    </div>
    `

    const clickBindings = () => {
        document.getElementById('quit')!.onclick = async () => {
            await quit()
        }

        document.getElementById('start-lesson')!.onclick = async () => {
            await invoke('set_counter', { stopAt: 50 })
            await getNextHiragana()
        }
    }

    return { html: html, setup: clickBindings }
}

async function quit() {
    await invoke('exit_app')
}
