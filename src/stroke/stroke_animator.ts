export class StrokeAnimator {
    private strokes: SVGPathElement[]

    constructor(svgElement: SVGSVGElement) {
        this.strokes = Array.from(svgElement.querySelectorAll('path'))
        this.prepareStrokes()
    }

    private prepareStrokes(): void {
        this.strokes.forEach((stroke) => {
            const length = stroke.getTotalLength()

            stroke.style.strokeDasharray = length.toString()
            stroke.style.strokeDashoffset = length.toString()
            stroke.style.fill = 'none'
        })
    }

    animateStroke(index: number, duration: number = 1000): Promise<void> {
        return new Promise((resolve) => {
            const stroke = this.strokes[index]
            const length = stroke.getTotalLength()

            stroke.animate(
                [{ strokeDashoffset: length }, { strokeDashoffset: 0 }],
                {
                    duration: duration,
                    easing: 'ease-in-out',
                    fill: 'forwards',
                },
            ).onfinish = (): void => {
                resolve()
            }
        })
    }

    async animateAll(
        strokeDuration: number = 1000,
        pauseBetween: number = 300,
    ): Promise<void> {
        for (let i = 0; i < this.strokes.length; i++) {
            await this.animateStroke(i, strokeDuration)
            if (i < this.strokes.length - 1) {
                await this.pause(pauseBetween)
            }
        }
    }

    private pause(ms: number): Promise<void> {
        return new Promise((resolve) => setTimeout(resolve, ms))
    }

    reset(): void {
        this.strokes.forEach((stroke) => {
            const length = stroke.getTotalLength()
            stroke.style.strokeDashoffset = length.toString()
            stroke.style.fill = 'none'
        })
    }
}
