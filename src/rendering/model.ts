export interface IElement {
    render(): string
    getCallbacks(): Array<() => Promise<void>>
}

export interface IOptions {
    callback?: () => Promise<void>
}

abstract class AbstractElement<T extends IOptions> implements IElement {
    protected options: T

    constructor(options: T) {
        this.options = options
    }

    abstract render(): string
    getCallbacks(): Array<() => Promise<void>> {
        return this.options.callback ? [this.options.callback] : []
    }
}

export interface TextOptions extends IOptions {
    readonly text: string
    readonly id?: string | undefined
}

abstract class TextElement extends AbstractElement<TextOptions> {
    protected abstract tag: string

    render(): string {
        const id = this.options.id ? ` id="${this.options.id}"` : ''
        return `<${this.tag}${id}>${this.options.text}</${this.tag}>`
    }
}

export class Heading1 extends TextElement {
    tag = 'h1'
}

export class Heading2 extends TextElement {
    tag = 'h2'
}

export class Heading3 extends TextElement {
    tag = 'h3'
}

export class Paragraph extends TextElement {
    tag = 'p'
}

export interface InputOptions extends IOptions {
    readonly id: string
    readonly placeholder: string
}

export class Input extends AbstractElement<InputOptions> {
    render(): string {
        return `<input type="text" id="${this.options.id}" placeholder="${this.options.placeholder}" autocomplete="off">`
    }
}

export interface ButtonOptions extends IOptions {
    readonly id: string
    readonly text: string
    readonly callback?: () => Promise<void>
}

export class Button extends AbstractElement<ButtonOptions> {
    render(): string {
        return `<button id="${this.options.id}">${this.options.text}</button>`
    }

    getCallbacks(): Array<() => Promise<void>> {
        if (!this.options.callback) {
            return []
        }

        return [
            async (): Promise<void> => {
                document.getElementById(this.options.id)!.onclick =
                    async (): Promise<void> => {
                        await this.options.callback!()
                    }
            },
        ]
    }
}

export interface DivOptions extends IOptions {
    readonly id: string
    readonly classes: Array<string>
    readonly styleOptions: undefined | string
    readonly elements: Array<IElement>
}

export class Div extends AbstractElement<DivOptions> {
    render(): string {
        const renderedElements = this.options.elements
            .map((element) => element.render())
            .join('')
        const classes =
            this.options.classes.length > 0
                ? ` class="${this.options.classes.join(' ')}"`
                : ''
        const style = this.options.styleOptions
            ? ` style="${this.options.styleOptions}"`
            : ''
        return `<div id="${this.options.id}"${classes}${style}>${renderedElements}</div>`
    }

    getCallbacks(): Array<() => Promise<void>> {
        const localCallbacks = this.options.callback
            ? [this.options.callback]
            : []
        const callbacks = this.options.elements.flatMap((element) =>
            element.getCallbacks(),
        )

        callbacks.push(...localCallbacks)
        return callbacks
    }
}

export interface FormOptions extends IOptions {
    readonly id: string
    readonly elements: Array<IElement>
}

export class Form extends AbstractElement<FormOptions> {
    render(): string {
        const renderedElements = this.options.elements
            .map((element) => element.render())
            .join('')
        return `<form id="${this.options.id}">${renderedElements}</form>`
    }

    getCallbacks(): Array<() => Promise<void>> {
        const localCallbacks = this.options.callback
            ? [this.options.callback]
            : []
        const callbacks = this.options.elements.flatMap((element) =>
            element.getCallbacks(),
        )

        callbacks.push(...localCallbacks)
        return callbacks
    }
}

export interface TableOptions extends IOptions {
    readonly classes?: Array<string>
    readonly headers: Array<string>
    readonly rows: Array<Array<IElement>>
}

export class Table extends AbstractElement<TableOptions> {
    render(): string {
        const renderedRows = this.options.rows
            .map((row) => {
                const renderedRow = row
                    .map((cell) => `<td>${cell.render()}</td>`)
                    .join('')
                return `<tr>${renderedRow}</tr>`
            })
            .join('')
        const renderedHeaders = this.options.headers
            .map((header) => `<th>${header}</th>`)
            .join('')
        const classes = this.options.classes
            ? ` class='${this.options.classes.join(' ')}'`
            : ''
        return `<table${classes}><tr>${renderedHeaders}</tr>${renderedRows}</table>`
    }
}
