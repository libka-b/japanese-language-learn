export interface Entry {
    readonly japanese: string
    readonly english: string
}

export interface Counter {
    readonly current: number
    readonly stop_at: number
}

export interface EntryCounter {
    readonly entry: Entry
    readonly counter: Counter
}

export interface EntryCount {
    readonly entry: Entry
    readonly count: number
}

export interface Stats {
    readonly total: number
    readonly incorrect: number
    readonly wrong: EntryCount[]
}

export interface LessonConfig {
    readonly name: string
    readonly path: string
}

export interface LessonGroup {
    readonly name: string
    readonly lesson_map: Record<string, LessonConfig>
    readonly lesson_order: Array<string>
}

export interface Config {
    readonly group_map: Record<string, LessonGroup>
    readonly group_order: Array<string>
}
