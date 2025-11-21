import './style.css'
import { createMenu } from './menu.ts'

export function getMainDivElement(): HTMLDivElement {
    return document.querySelector<HTMLDivElement>('#app')!
}

createMenu()
