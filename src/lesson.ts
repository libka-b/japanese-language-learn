import { invoke } from "@tauri-apps/api/core"
import { getMainDivElement } from "./main"
import { createMenu } from "./menu"
import type { EntryCounter, Entry } from "./types"

export async function getNextExercise(lessonName: string) {
    const entryCounter: EntryCounter | undefined = await invoke('next_lesson_entry', { lessonName: lessonName })
    if (!entryCounter) {
        createMenu()
        return
    }

    const counter = entryCounter.counter
    const entry = entryCounter.entry

    const html = `
    <h1>${entry.japanese}</h1>
    <h3>Exercise ${counter.current} out of ${counter.stop_at}</h3>
    <form id="form">
        <input type="text" id="input" placeholder="English transcript" autocomplete="off">
        <button type="button" id="submit">Submit</button>
    </form>
    <h3 id="result"></h3>
    `

    const mainDivElement = getMainDivElement()
    mainDivElement.innerHTML = html

    document.getElementById('submit')!.onclick = async () => { await onSubmit(lessonName, entry) }
    document.getElementById('form')!.addEventListener('submit', async (event) => {
        event.preventDefault()
        await onSubmit(lessonName, entry)
    })
    document.getElementById('input')?.focus()
}

async function onSubmit(lessonName: string, entry: Entry) {
    const input = (document.getElementById('input') as HTMLInputElement).value
    const isCorrect = input === entry.english ? true : false
    const message = isCorrect ? '✅ Correct!' : `❌ Incorrect! Should be '${entry.english}'`
    document.getElementById('result')!.innerHTML = message

    if (isCorrect) {
        await invoke('add_correct', { entry: { japanese: entry.japanese, english: entry.english }, lessonName: lessonName })
    } else {
        await invoke('add_incorrect', { entry: { japanese: entry.japanese, english: entry.english }, lessonName: lessonName })
    }

    await new Promise(resolve => setTimeout(resolve, 500))
    await getNextExercise(lessonName)
}
