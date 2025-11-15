import './style.css'
import { getButtons } from './menu.ts'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = await getButtons()
