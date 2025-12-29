import { getNextExercise } from './lesson'
import { createMenu } from './menu'
import { ConfigManager } from './config_manager'
import { generateLesson } from './agentic_lesson'
import { RendererBuilder } from './rendering/renderer'
import { DivBuilder } from './rendering/builder'

export async function createGroupMenu(
    configManager: ConfigManager,
    callback: (lessonOrder: Array<string>) => Promise<void>,
): Promise<void> {
    const divBuilder = new DivBuilder('menu', ['menu']).addButton({
        id: 'agentic-lesson',
        text: 'Agentic Lesson',
        callback: async (): Promise<void> => await generateLesson(),
    })

    configManager.getGroupOrder().forEach((groupName) => {
        divBuilder.addButton({
            id: groupName,
            text: `${capitalize(groupName)} group`,
            callback: async (): Promise<void> =>
                await callback(configManager.getLessonOrder(groupName)),
        })
    })

    divBuilder.addButton({
        id: 'main-menu',
        text: 'Back to main menu',
        callback: async (): Promise<void> => await createMenu(),
    })

    new RendererBuilder()
        .addDiv(divBuilder.build())
        .build()
        .renderAndRegisterCallbacks()
}

export async function createLessonMenu(
    lessonOrder: Array<string>,
): Promise<void> {
    const divBuilder = new DivBuilder('menu', ['menu'])

    lessonOrder.forEach((lessonName) => {
        divBuilder.addButton({
            id: lessonName,
            text: `${capitalize(lessonName)} lesson`,
            callback: async (): Promise<void> =>
                await getNextExercise(lessonName),
        })
    })

    divBuilder.addButton({
        id: 'main-menu',
        text: 'Back to main menu',
        callback: async (): Promise<void> => await createMenu(),
    })

    new RendererBuilder()
        .addDiv(divBuilder.build())
        .build()
        .renderAndRegisterCallbacks()
}

function capitalize(str: string): string {
    return str.charAt(0).toUpperCase() + str.slice(1)
}
