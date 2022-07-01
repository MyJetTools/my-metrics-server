var HtmlStaticElement = /** @class */ (function () {
    function HtmlStaticElement(el, toString) {
        this.el = el;
        this.toString = toString;
    }
    HtmlStaticElement.prototype.update = function (value) {
        if (this.value === undefined || this.value != value) {
            this.value = value;
            this.el.innerHTML = this.toString(value);
        }
    };
    return HtmlStaticElement;
}());
//# sourceMappingURL=HtmlStaticElement.js.map