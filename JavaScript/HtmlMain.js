var HtmlMain = /** @class */ (function () {
    function HtmlMain() {
    }
    HtmlMain.layout = function () {
        return '<div id="main"></div>' +
            HtmlStatusBar.layout();
    };
    HtmlMain.generateServicesList = function (services) {
        var result = '<table style="width:100%"><tr><td>';
        for (var _i = 0, _a = services.services; _i < _a.length; _i++) {
            var service = _a[_i];
            result += '<button type="button" class="btn btn-light" style="width:200px">' + service.id + '[' + this.micros_to_string(service.avg) + ']</button>';
        }
        return result + '</td><td style="width:100%"></td></tr></table>';
    };
    HtmlMain.micros_to_string = function (micros) {
        if (micros < 1000) {
            return micros + 'μs';
        }
        if (micros < 1000000) {
            return (micros / 1000) + 'ms';
        }
        return (micros / 1000000) + 's';
    };
    return HtmlMain;
}());
//# sourceMappingURL=HtmlMain.js.map