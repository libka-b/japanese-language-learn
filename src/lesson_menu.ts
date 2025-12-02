import { getNextExercise } from './lesson'
import { getMainDivElement } from './main'
import { createMenu } from './menu'
import { ConfigManager } from './config_manager'

export async function createLessonMenu(configManager: ConfigManager) {
    const lessonButtons = configManager.getLessonOrder().map(lessonName => {
        return `<button id="${lessonName}">${capitalize(lessonName)} lesson</button>`
    }).join('')

    const html = `
    <div class="menu">
        ${lessonButtons}
        <button id="main-menu">Back to main menu</button>
    </div>
    `

    getMainDivElement().innerHTML = html

    configManager.getLessonOrder().forEach(lessonName => {
        document.getElementById(lessonName)!.onclick = async () => {
            await getNextExercise(lessonName)
        }
    })

    document.getElementById('main-menu')!.onclick = async () => {
        await createMenu()
    }
}

function capitalize(str: string): string {
    return str.charAt(0).toUpperCase() + str.slice(1)
}
