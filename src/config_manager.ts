import type { Config, LessonType } from './types'

export class ConfigManager {
    private config: Config

    constructor(config: Config) {
        this.config = config
    }

    getGroupOrder = (): Array<string> => this.config.group_order
    getLessonType = (groupName: string): LessonType =>
        this.config.group_map[groupName]
}
