export interface Entry {
    readonly japanese: string
    readonly english: string
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
