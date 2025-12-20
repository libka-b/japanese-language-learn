import { invoke } from '@tauri-apps/api/core'
import type { AgenticLesson } from './types'
import { createMenu } from './menu'

export async function generateLesson(): Promise<void> {
    const agenticLesson: AgenticLesson = await invoke('generate_agentic_lesson')

    const html = `
    <h1>${agenticLesson.japanese_text}</h1>
    <form id="form">
        <input type="text" id="input" placeholder="English translation" autocomplete="off">
        <button type="button" id="submit">Submit</button>
    </form>
    <h3 id="result"></h3>
    `

    const mainDivElement = document.querySelector<HTMLDivElement>('#app')!
    mainDivElement.innerHTML = html

    document.getElementById('submit')!.onclick = async (): Promise<void> => {
        await createMenu()
    }
    document
        .getElementById('form')!
        .addEventListener('submit', async (event): Promise<void> => {
            event.preventDefault()
            await createMenu()
        })
}
