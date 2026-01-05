import { getNextCharacterExercise } from './character_lesson'
import { getNextVocabularyExercise } from './vocabulary_lesson'
import { createMenu } from './menu'
import { ConfigManager } from './config_manager'
import { generateAgenticLesson } from './agentic_lesson'
import { RendererBuilder } from './rendering/renderer'
import { DivBuilder } from './rendering/builder'
import { getCharacterLearningLesson } from './character_learning_lesson'
import type {
    LessonType,
    LessonGroup,
    LessonTypeEnum,
    CharacterLearningLesson,
} from './types'

function isAgenticExercise(
    lessonType: LessonType,
): lessonType is 'AgenticExercise' {
    return lessonType === 'AgenticExercise'
}

function isCharacterExercise(
    lessonType: LessonType,
): lessonType is { CharacterExercise: LessonGroup } {
    return typeof lessonType === 'object' && 'CharacterExercise' in lessonType
}

function isVocabularyExercise(
    lessonType: LessonType,
): lessonType is { VocabularyExercise: LessonGroup } {
    return typeof lessonType === 'object' && 'VocabularyExercise' in lessonType
}

function isCharacterLearningLesson(
    lessonType: LessonType,
): lessonType is { CharacterLearning: CharacterLearningLesson } {
    return typeof lessonType === 'object' && 'CharacterLearning' in lessonType
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

        if (isCharacterLearningLesson(lessonObj)) {
            divBuilder.addButton({
                id: 'character-learning-lesson',
                text: 'Character Leaning Lesson',
                callback: async (): Promise<void> =>
                    await callback(
                        lessonObj.CharacterLearning.lesson_order,
                        'CHARACTER_LEARNING_LESSON',
                    ),
            })
        }

        if (isAgenticExercise(lessonObj)) {
            divBuilder.addButton({
                id: 'agentic-lesson',
                text: 'Agentic lesson',
                callback: async (): Promise<void> =>
                    await generateAgenticLesson(),
            })
        }

        if (isCharacterExercise(lessonObj)) {
            divBuilder.addButton({
                id: groupName,
                text: `${capitalize(groupName)} group`,
                callback: async (): Promise<void> =>
                    await callback(
                        lessonObj.CharacterExercise.lesson_order,
                        'CHARACTER_EXERCISE',
                    ),
            })
        }

        if (isVocabularyExercise(lessonObj)) {
            divBuilder.addButton({
                id: groupName,
                text: `${capitalize(groupName)} group`,
                callback: async (): Promise<void> =>
                    await callback(
                        lessonObj.VocabularyExercise.lesson_order,
                        'VOCABULARY_EXERCISE',
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
                if (lessonTypeEnum == 'CHARACTER_EXERCISE') {
                    await getNextCharacterExercise(lessonName)
                }

                if (lessonTypeEnum == 'VOCABULARY_EXERCISE') {
                    await getNextVocabularyExercise(lessonName)
                }

                if (lessonTypeEnum == 'CHARACTER_LEARNING_LESSON') {
                    await getCharacterLearningLesson(lessonName)
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
