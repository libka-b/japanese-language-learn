import { invoke } from '@tauri-apps/api/core'
import type { Stats } from './types'
import { createMenu } from './menu'
import { RendererBuilder } from './rendering/renderer'
import { TableBuilder, DivBuilder } from './rendering/builder'

export async function showStats(lessonOrder: Array<string>): Promise<void> {
    const namedStats: Record<string, Stats> = await invoke('get_stats')

    const rendererBuilder = new RendererBuilder(
        async (): Promise<void> => toggleHide(),
    )

    lessonOrder.map((lessonName) => {
        const stats = namedStats[lessonName]
        const success = (1 - stats.incorrect / stats.total) * 100
        const fail = (stats.incorrect / stats.total) * 100

        const tableBuilder = new TableBuilder(['Character', 'Mistakes'])

        stats.wrong
            .sort((a, b) => b.count - a.count)
            .forEach((entry) =>
                tableBuilder.addRow([
                    entry.entry.japanese,
                    entry.count.toString(),
                ]),
            )

        rendererBuilder
            .addHeader2({ text: `${lessonName} statistics` })
            .addParagraph({ text: `Success rate: ${success}%` })
            .addDiv(
                new DivBuilder('success-bar', ['success-bar'])
                    .addDiv(
                        new DivBuilder(
                            'success',
                            ['success'],
                            `width: ${success}%`,
                        ).build(),
                    )
                    .addDiv(
                        new DivBuilder(
                            'fail',
                            ['fail'],
                            `width: ${fail}%`,
                        ).build(),
                    )
                    .build(),
            )
            .addDiv(
                new DivBuilder('details', ['details', 'hidden'])
                    .addTable(tableBuilder.build())
                    .build(),
            )
    })

    rendererBuilder
        .addButton({
            id: 'main-menu',
            text: 'Back to Main Menu',
            callback: async (): Promise<void> => await createMenu(),
        })
        .build()
        .renderAndRegisterCallbacks()
}

function toggleHide(): void {
    document.querySelectorAll('.success-bar').forEach((bar) => {
        bar.addEventListener('click', function () {
            bar.nextElementSibling?.classList.toggle('hidden')
        })
    })
}
