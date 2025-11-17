import './style.css'
import { createMenu } from './menu.ts'

export function getMainDivElement(): HTMLDivElement {
    return document.querySelector<HTMLDivElement>('#app')!
}

export function generateMainMenu() {
    const { html, setup } = createMenu()
    const mainDivElement = getMainDivElement()
    mainDivElement.innerHTML = html
    setup()
}

generateMainMenu()
