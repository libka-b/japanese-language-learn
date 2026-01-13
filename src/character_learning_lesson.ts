import { invoke } from '@tauri-apps/api/core'
import { createMenu } from './menu'
import type { CharacterEntry, CharacterEntryTable } from './types'
import { RendererBuilder } from './rendering/renderer'
import { DivBuilder, TableBuilder } from './rendering/builder'
import { Div, Svg } from './rendering/model'
import { StrokeAnimator } from './stroke/stroke_animator'

function emptyDiv(): Div {
    return new Div({
        id: '',
        classes: [],
        elements: [],
        styleOptions: undefined,
    })
}

function divFromCharacterEntry(
    characterEntry: CharacterEntry | undefined,
    lessonName: string,
): Div {
    if (!characterEntry) {
        return emptyDiv()
    }

    return new DivBuilder({
        id: characterEntry.japanese,
        callback: async (): Promise<void> => {
            document
                .getElementById(characterEntry.japanese)
                ?.addEventListener('click', () =>
                    callback(lessonName, characterEntry.japanese),
                )
        },
    })
        .addHeader1({ text: characterEntry.japanese })
        .addHeader3({ text: characterEntry.english })
        .build()
}

async function callback(lessonName: string, character: string): Promise<void> {
    const response = await fetch(`/hiragana/${character}.svg`)
    const svgText = await response.text()

    console.log(`SVG Text: ${svgText}`)

    new RendererBuilder()
        .addDiv(
            new DivBuilder({ id: '', classes: ['svg'] })
                .addSvg(new Svg({ rawSvg: svgText }))
                .build(),
        )
        .addButton({
            id: 'back',
            text: 'Back',
            callback: async () => getCharacterLearningLesson(lessonName),
        })
        .build()
        .renderAndRegisterCallbacks()

    const svgElement = document.querySelector('svg') as SVGSVGElement
    const animator = new StrokeAnimator(svgElement)

    animator.animateAll(800, 400)
}

export async function getCharacterLearningLesson(
    lessonName: string,
): Promise<void> {
    const characterEntryTable: CharacterEntryTable = await invoke(
        'get_character_table',
        { lessonName: lessonName },
    )

    const tableBuilder = new TableBuilder(['character-learning-table'])

    characterEntryTable.rows.forEach((row) => {
        tableBuilder.addRow(
            divFromCharacterEntry(row.col1, lessonName),
            divFromCharacterEntry(row.col2, lessonName),
            divFromCharacterEntry(row.col3, lessonName),
            divFromCharacterEntry(row.col4, lessonName),
            divFromCharacterEntry(row.col5, lessonName),
        )
    })

    new RendererBuilder()
        .addHeader1({ text: lessonName })
        .addTable(tableBuilder.build())
        .addButton({
            id: 'main-menu',
            text: 'Main Menu',
            callback: async () => await createMenu(),
        })
        .build()
        .renderAndRegisterCallbacks()
}
