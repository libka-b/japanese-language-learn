import { invoke } from "@tauri-apps/api/core"
import { generateMainMenu, getMainDivElement } from "./main"

interface Entry {
    readonly japanese: string
    readonly english: string
}

export async function getNextHiragana() {
    const entry: Entry | undefined = await invoke('next_hiragana_entry')
    if (!entry) {
        generateMainMenu()
        return
    }

    const html = `
    <h1>${entry.japanese}</h1>
    <form id="form">
        <input type="text" id="input" placeholder="English transcript" autocomplete="off">
        <button type="button" id="submit">Submit</button>
    </form>
    <h3 id="result"></h3>
    `

    const mainDivElement = getMainDivElement()
    mainDivElement.innerHTML = html

    document.getElementById('submit')!.onclick = async () => { await onSubmit(entry) }
    document.getElementById('form')!.addEventListener('submit', async (event) => {
        event.preventDefault()
        await onSubmit(entry)
    })
    document.getElementById('input')?.focus()
}

async function onSubmit(entry: Entry) {
    const input = (document.getElementById('input') as HTMLInputElement).value
    const isCorrect = input === entry.english ? true : false
    const message = isCorrect ? '✅ Correct!' : '❌ Incorrect!'
    document.getElementById('result')!.innerHTML = message

    await new Promise(resolve => setTimeout(resolve, 1000))
    await getNextHiragana()
}
