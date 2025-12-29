import { invoke } from '@tauri-apps/api/core'
import { configManager } from './main'
import { showStats } from './stats'
import { createGroupMenu, createLessonMenu } from './lesson_menu'
import { RendererBuilder } from './rendering/renderer'
import { DivBuilder } from './rendering/builder'

export function createMenu(): void {
    new RendererBuilder()
        .addDiv(
            new DivBuilder('main-menu', ['menu'])
                .addButton({
                    id: 'lessons',
                    text: 'Go to lessons',
                    callback: async () =>
                        await createGroupMenu(configManager, createLessonMenu),
                })
                .addButton({
                    id: 'view-stats',
                    text: 'View Stats',
                    callback: async () =>
                        await createGroupMenu(configManager, showStats),
                })
                .addButton({
                    id: 'quit',
                    text: 'Quit',
                    callback: async () => await quit(),
                })
                .build(),
        )
        .build()
        .renderAndRegisterCallbacks()
}

async function quit(): Promise<void> {
    await invoke('exit_app')
}
