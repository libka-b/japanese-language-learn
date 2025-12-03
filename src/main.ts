import './style.css'
import { createMenu } from './menu.ts'
import { invoke } from '@tauri-apps/api/core'
import { ConfigManager } from './config_manager.ts'
import type { Config } from './types.ts'

export function getMainDivElement(): HTMLDivElement {
    return document.querySelector<HTMLDivElement>('#app')!
}

async function loadConfig(): Promise<ConfigManager> {
    const config: Config = await invoke('get_config')
    return new ConfigManager(config)
}

export const configManager = await loadConfig()

createMenu()
