import { invoke } from '@tauri-apps/api/core'
import type { Stats } from './types'
import { createMenu } from './menu'
import { getMainDivElement } from './main'

export async function showStats(lessonOrder: Array<string>): Promise<void> {
    const namedStats: Record<string, Stats> = await invoke('get_stats')

    let html = lessonOrder
        .map((lessonName) => {
            const stats = namedStats[lessonName]
            const success = (1 - stats.incorrect / stats.total) * 100
            const fail = (stats.incorrect / stats.total) * 100
            const sortedWrongs = stats.wrong
                .sort((a, b) => b.count - a.count)
                .map(
                    (entry) =>
                        `<tr><th>${entry.entry.japanese}</th><th>${entry.count}</th></tr>`,
                )
                .join('')

            return `
            <h2>${lessonName} statistics</h2>
            Success rate: ${success}%
            <div class="success-bar">
                <div class="success" style="width: ${success}%"></div>
                <div class="fail" style="width: ${fail}%"></div>
            </div>
            <div id="details" class="details hidden">
                <table>
                    <tr>
                        <th>Character</th>
                        <th>Mistakes</th>
                        ${sortedWrongs}
                    </tr>
                </table>
            </div>
        `
        })
        .join('')

    html += `<button id="main-menu">Back to Main Menu</button>`

    getMainDivElement().innerHTML = html

    document.getElementById('main-menu')!.onclick = (): void => createMenu()

    document.querySelectorAll('.success-bar').forEach((bar) => {
        bar.addEventListener('click', function () {
            bar.nextElementSibling?.classList.toggle('hidden')
        })
    })
}
