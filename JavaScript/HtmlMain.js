var HtmlMain = /** @class */ (function () {
    function HtmlMain() {
    }
    HtmlMain.layout = function () {
        return '<div id="main"></div>' +
            HtmlStatusBar.layout();
    };
    HtmlMain.generateServicesList = function (services) {
        var result = '<table style="width:100%"><tr><td>';
        for (var _i = 0, _a = services.names; _i < _a.length; _i++) {
            var service = _a[_i];
            result += '<button type="button" class="btn btn-light" style="width:200px">' + service + '</button>';
        }
        return result + '</td><td style="width:100%"></td></tr></table>';
    };
    return HtmlMain;
}());
//# sourceMappingURL=HtmlMain.js.map