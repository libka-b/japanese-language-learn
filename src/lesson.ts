import { invoke } from '@tauri-apps/api/core'
import { createMenu } from './menu'
import type { EntryCounter, Entry } from './types'
import { RendererBuilder } from './rendering/renderer'
import { FormBuilder } from './rendering/builder'

export async function getNextExercise(lessonName: string): Promise<void> {
    const entryCounter: EntryCounter | undefined = await invoke(
        'next_lesson_entry',
        { lessonName: lessonName },
    )
    if (!entryCounter) {
        createMenu()
        return
    }

    const counter = entryCounter.counter
    const entry = entryCounter.entry

    new RendererBuilder()
        .addHeader1({ text: entry.japanese })
        .addHeader3({
            text: `Exercise ${counter.current} out of ${counter.stop_at}`,
        })
        .addForm(
            new FormBuilder(
                'form',
                async (): Promise<void> => callback(lessonName, entry),
            )
                .addInput({ id: 'input', placeholder: 'English transcript' })
                .addButton({ id: 'submit', text: 'Submit' })
                .build(),
        )
        .addHeader3({ text: '', id: 'result' })
        .build()
        .renderAndRegisterCallbacks()
}

async function onSubmit(lessonName: string, entry: Entry): Promise<void> {
    const input = (document.getElementById('input') as HTMLInputElement).value
    const isCorrect = input === entry.english ? true : false
    const message = isCorrect
        ? '✅ Correct!'
        : `❌ Incorrect! Should be '${entry.english}'`
    document.getElementById('result')!.innerHTML = message

    if (isCorrect) {
        await invoke('add_correct', {
            entry: { japanese: entry.japanese, english: entry.english },
            lessonName: lessonName,
        })
    } else {
        await invoke('add_incorrect', {
            entry: { japanese: entry.japanese, english: entry.english },
            lessonName: lessonName,
        })
    }

    await new Promise((resolve) => setTimeout(resolve, 500))
    await getNextExercise(lessonName)
}

function callback(lessonName: string, entry: Entry): void {
    document
        .getElementById('form')!
        .addEventListener('submit', async (event): Promise<void> => {
            event.preventDefault()
            await onSubmit(lessonName, entry)
        })
    document.getElementById('input')?.focus()
}
