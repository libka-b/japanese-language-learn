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

    abstract build(): T
}

export class DivBuilder extends ContainerBuilder<Div> {
    private id: string
    private classes: Array<string>
    private styleOptions: undefined | string

    constructor(
        id: string,
        classes: Array<string>,
        styleOptions?: undefined | string,
    ) {
        super()
        this.id = id
        this.classes = classes
        this.styleOptions = styleOptions
    }

    build(): Div {
        return new Div({
            id: this.id,
            classes: this.classes,
            styleOptions: this.styleOptions,
            elements: this.elements,
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
    private rows: Array<Array<string>> = []

    constructor(headers: Array<string>) {
        this.headers = headers
    }

    addRow(row: Array<string>): TableBuilder {
        this.rows.push(row)
        return this
    }

    build(): Table {
        return new Table({
            headers: this.headers,
            rows: this.rows,
        })
    }
}
