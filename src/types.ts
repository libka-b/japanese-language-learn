export interface CharacterEntry {
    readonly japanese: string
    readonly english: string
}

export interface VocabularyEntry {
    readonly japanese: string
    readonly english: string
    readonly pronunciation: string
}

export interface Counter {
    readonly current: number
    readonly stop_at: number
}

export interface EntryCounter<T> {
    readonly entry: T
    readonly counter: Counter
}

export interface EntryCount<T> {
    readonly entry: T
    readonly count: number
}

export interface Stats<T> {
    readonly total: number
    readonly incorrect: number
    readonly wrong: EntryCount<T>[]
}

export interface LessonConfig {
    readonly name: string
    readonly path: string
}

export interface LessonGroup {
    readonly lesson_type: 'Agentic' | 'Character' | 'Vocabulary'
    readonly name: string
    readonly lesson_map: Record<string, LessonConfig>
    readonly lesson_order: Array<string>
}

export type LessonType =
    | 'Agentic'
    | { Character: LessonGroup }
    | { Vocabulary: LessonGroup }

export interface Config {
    readonly group_map: Record<string, LessonType>
    readonly group_order: Array<string>
}

export type LessonTypeEnum = 'AGENTIC' | 'CHARACTER' | 'VOCABULARY'

export interface AgenticLesson {
    readonly japanese_text: string
}

export interface Translation {
    readonly original_text: string
    readonly translation: string
    readonly correction: string
    readonly mistakes: string
    readonly suggestions: string
    readonly mark: number
}
