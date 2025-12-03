import type { Config } from './types'

export class ConfigManager {
    private config: Config

    constructor(config: Config) {
        this.config = config
    }

    getGroupOrder = () => this.config.group_order
    getLessonOrder = (groupName: string) => this.config
        .group_map[groupName]
        .lesson_order
}
