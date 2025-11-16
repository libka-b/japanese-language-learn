import './style.css'
import { createMenu } from './menu.ts'

function generateMainMenu() {
    const { html, setup } = createMenu()
    document.querySelector<HTMLDivElement>('#app')!.innerHTML = html
    setup()
}

generateMainMenu()
