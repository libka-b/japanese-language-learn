import {
    Heading1,
    Heading2,
    Heading3,
    Paragraph,
    Button,
    Div,
    Form,
    Table,
    Input,
    Svg,
} from './model'
import type {
    ButtonOptions,
    IElement,
    InputOptions,
    TextOptions,
} from './model'

interface Builder<T> {
    build(): T
}

export abstract class ContainerBuilder<T> implements Builder<T> {
    protected elements: Array<IElement> = []

    addHeader1(options: TextOptions): ContainerBuilder<T> {
        this.elements.push(new Heading1(options))
        return this
    }

    addHeader2(options: TextOptions): ContainerBuilder<T> {
        this.elements.push(new Heading2(options))
        return this
    }

    addHeader3(options: TextOptions): ContainerBuilder<T> {
        this.elements.push(new Heading3(options))
        return this
    }

    addParagraph(options: TextOptions): ContainerBuilder<T> {
        this.elements.push(new Paragraph(options))
        return this
    }

    addInput(options: InputOptions): ContainerBuilder<T> {
        this.elements.push(new Input(options))
        return this
    }

    addButton(options: ButtonOptions): ContainerBuilder<T> {
        this.elements.push(new Button(options))
        return this
    }

    addDiv(div: Div): ContainerBuilder<T> {
        this.elements.push(div)
        return this
    }

    addForm(form: Form): ContainerBuilder<T> {
        this.elements.push(form)
        return this
    }

    addTable(table: Table): ContainerBuilder<T> {
        this.elements.push(table)
        return this
    }

    addSvg(svg: Svg): ContainerBuilder<T> {
        this.elements.push(svg)
        return this
    }

    abstract build(): T
}

export interface DivBuilderOptions {
    readonly id: string
    readonly classes?: Array<string>
    readonly styleOptions?: string
    readonly callback?: () => Promise<void>
}

export class DivBuilder extends ContainerBuilder<Div> {
    private id: string
    private classes: Array<string>
    private styleOptions: undefined | string
    private callback: undefined | (() => Promise<void>)

    constructor(divBuilderOptions: DivBuilderOptions) {
        super()
        this.id = divBuilderOptions.id
        this.classes = divBuilderOptions.classes
            ? divBuilderOptions.classes
            : []
        this.styleOptions = divBuilderOptions.styleOptions
        this.callback = divBuilderOptions.callback
    }

    build(): Div {
        return new Div({
            id: this.id,
            classes: this.classes,
            styleOptions: this.styleOptions,
            elements: this.elements,
            callback: this.callback,
        })
    }
}

export class FormBuilder extends ContainerBuilder<Form> {
    private id: string
    private callback?: () => Promise<void>

    constructor(id: string, callback?: () => Promise<void>) {
        super()
        this.id = id
        this.callback = callback
    }

    build(): Form {
        return new Form({
            id: this.id,
            elements: this.elements,
            callback: this.callback,
        })
    }
}

export class TableBuilder implements Builder<Table> {
    private headers: Array<string>
    private rows: Array<Array<IElement>> = []
    private classes: Array<string> | undefined

    constructor(classes: Array<string>, ...headers: string[]) {
        this.headers = headers
        this.classes = classes
    }

    addRow(...row: IElement[]): TableBuilder {
        this.rows.push(row)
        return this
    }

    build(): Table {
        return new Table({
            classes: this.classes,
            headers: this.headers,
            rows: this.rows,
        })
    }
}
