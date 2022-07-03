var Dialog = /** @class */ (function () {
    function Dialog() {
    }
    Dialog.show = function (header, html) {
        var el = document.getElementById('service-overview');
        this.prevHtml = el.innerHTML;
        el.innerHTML = '<div><table><tr><td><button class="btn btn-light" onclick="Dialog.pressBack()">Back</button></td><td><h4>' + header + '</h4></td></table></div>' + html;
    };
    Dialog.override = function (header, html) {
        var el = document.getElementById('service-overview');
        el.innerHTML = '<div><table><tr><td><button class="btn btn-light" onclick="Dialog.pressBack()">Back</button></td><td><h4>' + header + '</h4></td></table></div>' + html;
    };
    Dialog.pressBack = function () {
        var el = document.getElementById('service-overview');
        el.innerHTML = this.prevHtml;
    };
    return Dialog;
}());
//# sourceMappingURL=Dialog.js.map