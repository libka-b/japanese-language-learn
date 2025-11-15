import { invoke } from '@tauri-apps/api/core'
import type { Button } from './types'

export async function getButtons(): Promise<string> {
    const buttons: Button[] = await invoke('get_menu_buttons')

    const mapped = buttons.map((button) => `<li><button id="${button.id}">${button.text}</button"></li>`)

    return `<ul>${mapped.join('')}</ul>`
}
