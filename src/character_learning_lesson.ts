import { invoke } from '@tauri-apps/api/core'
import { createMenu } from './menu'
import type { CharacterEntry, CharacterEntryTable } from './types'
import { RendererBuilder } from './rendering/renderer'
import { DivBuilder, TableBuilder } from './rendering/builder'
import { Div } from './rendering/model'

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
): Div {
    if (!characterEntry) {
        return emptyDiv()
    }

    return new DivBuilder('', [])
        .addHeader1({ text: characterEntry.japanese })
        .addHeader3({ text: characterEntry.english })
        .build()
}

export async function getCharacterLearningLesson(
    lessonName: string,
): Promise<void> {
    const characterEntryTable: CharacterEntryTable = await invoke(
        'get_character_table',
        { lessonName: lessonName },
    )

    const tableBuilder = new TableBuilder([])

    characterEntryTable.rows.forEach((row) => {
        tableBuilder.addRow([
            divFromCharacterEntry(row.col1),
            divFromCharacterEntry(row.col2),
            divFromCharacterEntry(row.col3),
            divFromCharacterEntry(row.col4),
            divFromCharacterEntry(row.col5),
        ])
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
