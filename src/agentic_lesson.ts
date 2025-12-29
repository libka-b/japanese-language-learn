import { invoke } from '@tauri-apps/api/core'
import type { AgenticLesson, Translation } from './types'
import { createMenu } from './menu'
import { RendererBuilder } from './rendering/renderer'
import { FormBuilder } from './rendering/builder'

export async function renderSetApiKeyPage(): Promise<void> {
    new RendererBuilder()
        .addHeader1({ text: 'Set Gemini API Key' })
        .addForm(
            new FormBuilder(
                'form',
                async (): Promise<void> => apiKeyFormCallback(),
            )
                .addInput({ id: 'input', placeholder: 'API Key' })
                .addButton({ id: 'submit', text: 'Submit' })
                .build(),
        )
        .build()
        .renderAndRegisterCallbacks()
}

function apiKeyFormCallback(): void {
    document
        .getElementById('form')!
        .addEventListener('submit', async (event): Promise<void> => {
            event.preventDefault()
            await setApiKey()
            await createMenu()
        })
    document.getElementById('input')?.focus()
}

async function setApiKey(): Promise<void> {
    const key = (document.getElementById('input') as HTMLInputElement).value
    await invoke('set_api_key', { key: key })
}

export async function generateLesson(): Promise<void> {
    let agenticLesson: AgenticLesson
    try {
        agenticLesson = await invoke('generate_agentic_lesson')
    } catch {
        return renderSetApiKeyPage()
    }

    new RendererBuilder()
        .addHeader1({ text: agenticLesson.japanese_text })
        .addForm(
            new FormBuilder(
                'form',
                async (): Promise<void> =>
                    lessonCallback(agenticLesson.japanese_text),
            )
                .addInput({ id: 'input', placeholder: 'English translation' })
                .addButton({ id: 'submit', text: 'submit' })
                .build(),
        )
        .addHeader3({ text: '', id: 'result' })
        .build()
        .renderAndRegisterCallbacks()
}

export async function validateTranslation(originalText: string): Promise<void> {
    const input = (document.getElementById('input') as HTMLInputElement).value
    const translation: Translation = await invoke(
        'validate_translation_lesson',
        { original: originalText, translation: input },
    )

    new RendererBuilder()
        .addHeader2({ text: `Original text '${translation.original_text}'` })
        .addHeader3({ text: `Translation: '${translation.translation}'` })
        .addHeader3({ text: `Correction: '${translation.correction}'` })
        .addHeader3({ text: `Mistakes: '${translation.mistakes}'` })
        .addHeader3({ text: `Suggestions: '${translation.suggestions}'` })
        .addHeader3({ text: `Mark: '${translation.mark}'` })
        .addButton({
            id: 'main-menu',
            text: 'Back to main menu',
            callback: async (): Promise<void> => await createMenu(),
        })
        .build()
        .renderAndRegisterCallbacks()
}

function lessonCallback(japaneseText: string): void {
    document
        .getElementById('form')!
        .addEventListener('submit', async (event): Promise<void> => {
            event.preventDefault()
            await validateTranslation(japaneseText)
        })
    document.getElementById('input')?.focus()
}
