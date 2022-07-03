var HtmlMain = /** @class */ (function () {
    function HtmlMain() {
    }
    HtmlMain.layout = function () {
        return '<div id="main"></div>' +
            HtmlStatusBar.layout();
    };
    HtmlMain.generateServicesList = function (services) {
        var result = '<table style="width:100%; height:100%"><tr style="vertical-align: top;"><td>';
        for (var _i = 0, _a = services.services.sort(function (a, b) { return b.avg > a.avg ? 1 : -1; }); _i < _a.length; _i++) {
            var service = _a[_i];
            result += '<button id="app-' + service.id + '" service="' + service.id + '" type="button" class="btn btn-light" style="width:300px" onclick="AppSelector.serviceSelected(this)">' + service.id + '<div style="font-size:8px">' + this.micros_to_string(service.avg) + '<div></button>';
        }
        return result + '</td><td id="service-overview" style="width:100%"></td></tr></table>';
    };
    HtmlMain.generateServiceOverview = function (appId, overviews) {
        var result = '<table class="table table-striped" style="font-size:10px"><tr><th>Data</th><th>Max</th><th>Min</th><th>Avg</th><th>Success</th><th>Errors</th><th>Total</th><th></th></tr>';
        for (var _i = 0, _a = overviews.data.sort(function (a, b) { return b.avg > a.avg ? 1 : -1; }); _i < _a.length; _i++) {
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
    HtmlMain.generateMetrics = function (metrics) {
        var result = '<table class="table table-striped" style="font-size:10px"><tr><th>Started</th><th>Duration</th><th>Message</th><th>Ip</th><th></th></tr>';
        for (var _i = 0, _a = metrics.metrics.sort(function (a, b) { return a.started > b.started ? 1 : -1; }); _i < _a.length; _i++) {
            var metric = _a[_i];
            var date = new Date(metric.started / 1000);
            var data = "";
            if (metric.success) {
                data = '<span style="color:green">' + metric.success + '</span>';
            }
            if (metric.error) {
                data = '<span style="color:red">' + metric.error + '</span>';
            }
            result += '<tr><td><div>' + date.toLocaleString() + '</div><div>' + date.toISOString() + '</div></td><td>' + this.micros_to_string(metric.duration) + '</td><td>' + data + '</td><td>' + metric.ip + '</td><td><button data-process-id="' + metric.id + '" class="btn btn-light btn-sm" onclick="AppSelector.showByProcessId(this)">Show</button></td></tr>';
        }
        return result + '</table>';
    };
    HtmlMain.generateMetricsWithDuration = function (metrics) {
        if (metrics.metrics.length == 0) {
            return "No Data";
        }
        var _a = this.getMaximumDuration(metrics), min = _a.min, max = _a.max;
        var maxDuration = max - min;
        var result = '<table class="table table-striped" style="font-size:10px"><tr><th>Started</th><th>Duration</th><th>Message</th><th>Ip</th><th></th></tr>';
        for (var _i = 0, _b = metrics.metrics.sort(function (a, b) { return a.started > b.started ? 1 : -1; }); _i < _b.length; _i++) {
            var metric = _b[_i];
            var date = new Date(metric.started / 1000);
            var data = "";
            if (metric.success) {
                data = '<span style="color:green">' + metric.success + '</span>';
            }
            if (metric.error) {
                data = '<span style="color:red">' + metric.error + '</span>';
            }
            var pad = metric.started - min / maxDuration * 100;
            var width = metric.duration / maxDuration * 100;
            result += '<tr><td><div>' + date.toLocaleString() + '</div><div>' + date.toISOString() + '</div></td><td>' + this.micros_to_string(metric.duration) + '</td><td>' + data + '</td><td>' + metric.ip + '</td><td><button data-process-id="' + metric.id + '" class="btn btn-light btn-sm" onclick="AppSelector.showByProcessId(this)">Show</button></td></tr>'
                + '<tr><td colspan="5"><span style="padding:' + pad.toFixed(2) + '%;width:' + width.toFixed(2) + '%;height:20px; color: blue; background:blue;"></span></td></tr>';
        }
        return result + '</table>';
    };
    HtmlMain.getMaximumDuration = function (metrics) {
        var min = metrics.metrics[0].started;
        var max = metrics.metrics[0].started + metrics.metrics[0].duration;
        for (var _i = 0, _a = metrics.metrics; _i < _a.length; _i++) {
            var metric = _a[_i];
            if (min > metric.started) {
                min = metric.started;
            }
            var ended = metric.started + metric.duration;
            if (max < ended) {
                max = ended;
            }
            return { min: min, max: max };
        }
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