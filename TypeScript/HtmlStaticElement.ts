
class HtmlStaticElement<T> {
    value: T;
    el: HTMLElement;
    toString: (value: T) => string

    constructor(el: HTMLElement, toString: (value: T) => string) {
        this.el = el;
        this.toString = toString;
    }

    public update(value: T) {
        if (this.value === undefined || this.value != value) {
            this.value = value;
            this.el.innerHTML = this.toString(value);
        }
    }

}