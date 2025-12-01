import { getNextExercise } from './lesson'
import { getMainDivElement } from './main'
import { createMenu } from './menu'

export async function createLessonMenu() {
    const html = `
    <div class="menu">
        <button id="hiragana-lesson">Hiragana lesson</button>
        <button id="hiragana-plus-lesson">Hiragana Plus lesson</button>
        <button id="katakana-lesson">Katakana lesson</button>
        <button id="katakana-plus-lesson">Katakana Plus lesson</button>
        <button id="main-menu">Back to main menu</button>
    </div>
    `

    getMainDivElement().innerHTML = html

    document.getElementById('hiragana-lesson')!.onclick = async () => {
        await getNextExercise('hiragana')
    }

    document.getElementById('hiragana-plus-lesson')!.onclick = async () => {
        await getNextExercise('hiragana-plus')
    }

    document.getElementById('katakana-lesson')!.onclick = async () => {
        await getNextExercise('katakana')
    }

    document.getElementById('katakana-plus-lesson')!.onclick = async () => {
        await getNextExercise('katakana-plus')
    }

    document.getElementById('main-menu')!.onclick = async () => {
        await createMenu()
    }
}