import { getNextExercise } from './lesson'
import { getMainDivElement } from './main'
import { createMenu } from './menu'
import { ConfigManager } from './config_manager'

export async function createGroupMenu(
    configManager: ConfigManager,
    callback: (lessonOrder: Array<string>) => Promise<void>,
): Promise<void> {
    const groupButtons = configManager
        .getGroupOrder()
        .map((groupName) => {
            return `<button id="${groupName}">${capitalize(groupName)} group</button>`
        })
        .join('')

    const html = `
    <div class="menu">
        ${groupButtons}
        <button id="main-menu">Back to main menu</button>
    </div>
    `

    getMainDivElement().innerHTML = html

    configManager.getGroupOrder().forEach((groupName) => {
        document.getElementById(groupName)!.onclick =
            async (): Promise<void> => {
                await callback(configManager.getLessonOrder(groupName))
            }
    })

    document.getElementById('main-menu')!.onclick = async (): Promise<void> => {
        await createMenu()
    }
}

export async function createLessonMenu(
    lessonOrder: Array<string>,
): Promise<void> {
    const lessonButtons = lessonOrder
        .map((lessonName) => {
            return `<button id="${lessonName}">${capitalize(lessonName)} lesson</button>`
        })
        .join('')

    const html = `
    <div class="menu">
        ${lessonButtons}
        <button id="main-menu">Back to main menu</button>
    </div>
    `

    getMainDivElement().innerHTML = html

    lessonOrder.forEach((lessonName) => {
        document.getElementById(lessonName)!.onclick =
            async (): Promise<void> => {
                await getNextExercise(lessonName)
            }
    })

    document.getElementById('main-menu')!.onclick = async (): Promise<void> => {
        await createMenu()
    }
}

function capitalize(str: string): string {
    return str.charAt(0).toUpperCase() + str.slice(1)
}
