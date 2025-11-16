import { invoke } from '@tauri-apps/api/core'

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
    }

    return { html: html, setup: clickBindings }
}

async function quit() {
    await invoke('exit_app')
}
