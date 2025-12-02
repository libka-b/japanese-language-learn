import './style.css'
import { createMenu } from './menu.ts'
import { invoke } from '@tauri-apps/api/core'
import { ConfigManager } from './config_manager.ts'

export function getMainDivElement(): HTMLDivElement {
    return document.querySelector<HTMLDivElement>('#app')!
}

async function loadConfig(): Promise<ConfigManager> {
    const config: Array<string> = await invoke('get_config')
    return new ConfigManager(config)
}

export const configManager = await loadConfig()

createMenu()
