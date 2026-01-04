import { getNextCharacterExercise } from './character_lesson'
import { getNextVocabularyExercise } from './vocabulary_lesson'
import { createMenu } from './menu'
import { ConfigManager } from './config_manager'
import { generateAgenticLesson } from './agentic_lesson'
import { RendererBuilder } from './rendering/renderer'
import { DivBuilder } from './rendering/builder'
import type { LessonType, LessonGroup, LessonTypeEnum } from './types'

function isAgentic(lessonType: LessonType): lessonType is 'Agentic' {
    return lessonType === 'Agentic'
}

function isCharacter(
    lessonType: LessonType,
): lessonType is { Character: LessonGroup } {
    return typeof lessonType === 'object' && 'Character' in lessonType
}

function isVocabulary(
    lessonType: LessonType,
): lessonType is { Vocabulary: LessonGroup } {
    return typeof lessonType === 'object' && 'Vocabulary' in lessonType
}

export async function createGroupMenu(
    configManager: ConfigManager,
    callback: (
        lessonOrder: Array<string>,
        lessonTypeEnum: LessonTypeEnum,
    ) => Promise<void>,
): Promise<void> {
    const divBuilder = new DivBuilder('menu', ['menu'])

    configManager.getGroupOrder().forEach((groupName) => {
        const lessonObj = configManager.getLessonType(groupName)
        if (isAgentic(lessonObj)) {
            divBuilder.addButton({
                id: 'agentic-lesson',
                text: 'Agentic lesson',
                callback: async (): Promise<void> =>
                    await generateAgenticLesson(),
            })
        }

        if (isCharacter(lessonObj)) {
            divBuilder.addButton({
                id: groupName,
                text: `${capitalize(groupName)} group`,
                callback: async (): Promise<void> =>
                    await callback(
                        lessonObj.Character.lesson_order,
                        'CHARACTER',
                    ),
            })
        }

        if (isVocabulary(lessonObj)) {
            divBuilder.addButton({
                id: groupName,
                text: `${capitalize(groupName)} group`,
                callback: async (): Promise<void> =>
                    await callback(
                        lessonObj.Vocabulary.lesson_order,
                        'VOCABULARY',
                    ),
            })
        }
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
    lessonTypeEnum: LessonTypeEnum,
): Promise<void> {
    const divBuilder = new DivBuilder('menu', ['menu'])

    lessonOrder.forEach((lessonName) => {
        divBuilder.addButton({
            id: lessonName,
            text: `${capitalize(lessonName)} lesson`,
            callback: async (): Promise<void> => {
                if (lessonTypeEnum == 'CHARACTER') {
                    await getNextCharacterExercise(lessonName)
                }

                if (lessonTypeEnum == 'VOCABULARY') {
                    await getNextVocabularyExercise(lessonName)
                }
            },
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
