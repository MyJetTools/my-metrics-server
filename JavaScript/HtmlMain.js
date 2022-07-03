var HtmlMain = /** @class */ (function () {
    function HtmlMain() {
    }
    HtmlMain.layout = function () {
        return '<div id="main"></div>' +
            HtmlStatusBar.layout();
    };
    HtmlMain.generateServicesList = function (services) {
        var result = '<table style="width:100%; height:100%"><tr style="vertical-align: top;"><td>';
        for (var _i = 0, _a = services.services; _i < _a.length; _i++) {
            var service = _a[_i];
            result += '<button id="app-' + service.id + '" service="' + service.id + '" type="button" class="btn btn-light" style="width:300px" onclick="AppSelector.serviceSelected(this)">' + service.id + '<div style="font-size:8px">' + this.micros_to_string(service.avg) + '<div></button>';
        }
        return result + '</td><td id="service-overview" style="width:100%"></td></tr></table>';
    };
    HtmlMain.generateServiceOverview = function (appId, overviews) {
        var result = '<table class="table table-striped" style="font-size:10px"><tr><th>Data</th><th>Max</th><th>Min</th><th>Avg</th><th>Success</th><th>Errors</th><th>Total</th><th></th></tr>';
        for (var _i = 0, _a = overviews.data; _i < _a.length; _i++) {
            var overview = _a[_i];
            var errors = overview.error.toFixed(0);
            if (overview.error > 0) {
                errors = '<span style="color:red">' + errors + '</span>';
            }
            result += '<tr><td>' + overview.data + '</td><td>' + this.micros_to_string(overview.max) + '</td><td>' + this.micros_to_string(overview.min) + '</td><td>' + this.micros_to_string(overview.avg) + '</td><td>' + overview.success + '</td><td>' + overview.error + '</td><td>' + overview.total + '</td><td>' +
                '<button data-app="' + appId + '" data-app-data="' + overview.data + '" class="btn btn-light btn-sm" onclick="AppSelector.expandMetrics(this)">Expand</button></td></tr>';
        }
        return result + '</table>';
    };
    HtmlMain.generateMetrics = function (id, data, metrics) {
        var result = '<h3>' + id + '/' + data + ' < /h3><table class="table table-striped" style="font-size:10px"><tr><th>Started</th > <th>Duration < /th><th>Message</th > </tr>';
        for (var _i = 0, _a = metrics.metrics.sort(function (a, b) { return b.started > a.started ? 1 : -1; }); _i < _a.length; _i++) {
            var metric = _a[_i];
            var date = new Date(metric.started / 1000);
            var data_1 = "";
            if (metric.success) {
                data_1 = '<span style="color:green">' + metric.success + '</span>';
            }
            if (metric.error) {
                data_1 = '<span style="color:red">' + metric.error + '</span>';
            }
            result += '<tr><td>' + date.toLocaleString() + '</td><td>' + this.micros_to_string(metric.duration) + '</td><td>' + data_1 + '</td></tr>';
        }
        return result + '</table>';
    };
    HtmlMain.micros_to_string = function (micros) {
        if (micros < 1000) {
            return micros + 'micros';
        }
        if (micros < 1000000) {
            return (micros / 1000) + 'ms';
        }
        return (micros / 1000000) + 's';
    };
    return HtmlMain;
}());
//# sourceMappingURL=HtmlMain.js.map