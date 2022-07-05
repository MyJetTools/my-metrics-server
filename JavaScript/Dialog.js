var Dialog = /** @class */ (function () {
    function Dialog() {
    }
    Dialog.show = function (header, html) {
        var el = document.getElementById('service-overview');
        window.history.pushState({ div: 'service-overview', content: el.innerHTML }, "Details", '#service-overview');
        el.innerHTML = '<div><table><tr><td><button class="btn btn-light" onclick="Dialog.pressBack()">Back</button></td><td><h4>' + header + '</h4></td></table></div>' + html;
    };
    Dialog.pressBack = function () {
        window.history.back();
    };
    return Dialog;
}());
//# sourceMappingURL=Dialog.js.map