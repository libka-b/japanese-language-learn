import { invoke } from '@tauri-apps/api/core'
import type { AgenticLesson, Translation } from './types'
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
        await validateTranslation(agenticLesson.japanese_text)
    }
    document
        .getElementById('form')!
        .addEventListener('submit', async (event): Promise<void> => {
            event.preventDefault()
            await validateTranslation(agenticLesson.japanese_text)
        })
}

export async function validateTranslation(originalText: string): Promise<void> {
    const input = (document.getElementById('input') as HTMLInputElement).value
    const translation: Translation = await invoke(
        'validate_translation_lesson',
        { original: originalText, translation: input },
    )

    const html = `
    <h2>Original text '${translation.original_text}'</h2>
    <h3>Translation: '${translation.translation}'</h3>
    <h3>Correction: '${translation.correction}'</h3>
    <h3>Mistakes: '${translation.mistakes}'</h3>
    <h3>Suggestions: '${translation.suggestions}'</h3>
    <h3>Mark: '${translation.mark}'</h3>
    <button id="main-menu">Back to main menu</button>
    `

    const mainDivElement = document.querySelector<HTMLDivElement>('#app')!
    mainDivElement.innerHTML = html

    document.getElementById('main-menu')!.onclick = async (): Promise<void> => {
        await createMenu()
    }
}
