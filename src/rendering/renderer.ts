import { ContainerBuilder } from './builder'
import { type IElement } from './model'

export class RendererBuilder extends ContainerBuilder<Renderer> {
    private callback?: () => Promise<void>

    constructor(callback?: () => Promise<void>) {
        super()
        this.callback = callback
    }

    build(): Renderer {
        return new Renderer(this.elements, this.callback)
    }
}

export class Renderer {
    private elements: Array<IElement>
    private callback?: () => Promise<void>

    constructor(elements: Array<IElement>, callback?: () => Promise<void>) {
        this.elements = elements
        this.callback = callback
    }

    renderAndRegisterCallbacks(): void {
        this.render()
        this.registerCallbacks()
    }

    private render(): void {
        getMainDivElement().innerHTML = this.elements
            .map((element) => element.render())
            .join('')
    }

    private registerCallbacks(): void {
        this.elements
            .flatMap((element) => element.getCallbacks())
            .forEach((callback) => callback())

        this.callback?.()
    }
}

function getMainDivElement(): HTMLDivElement {
    return document.querySelector<HTMLDivElement>('#app')!
}
