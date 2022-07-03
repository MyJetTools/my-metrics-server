var Dialog = /** @class */ (function () {
    function Dialog() {
    }
    Dialog.show = function (html) {
        var el = document.getElementById('service-overview');
        this.prevHtml = el.innerHTML;
        el.innerHTML = '<div><button class="btn btn-light" onclick="Dialog.pressBack()">Back</button></div>' + html;
    };
    Dialog.pressBack = function () {
        var el = document.getElementById('service-overview');
        el.innerHTML = this.prevHtml;
    };
    return Dialog;
}());
//# sourceMappingURL=Dialog.js.map