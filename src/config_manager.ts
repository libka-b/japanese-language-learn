export class ConfigManager {
    private lessonOrder: Array<string>

    constructor(lessonOrder: Array<string>) {
        this.lessonOrder = lessonOrder
    }

    getLessonOrder(): Array<string> {
        return this.lessonOrder
    }
}
