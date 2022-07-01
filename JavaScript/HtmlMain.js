var HtmlMain = /** @class */ (function () {
    function HtmlMain() {
    }
    HtmlMain.layout = function () {
        return '<div id="main"></div>' +
            HtmlStatusBar.layout();
    };
    HtmlMain.generateContent = function (status) {
        var tableTop = '<table class="table table-striped">' +
            '<thead><tr><th>Id</th><th>Name</th><th>Version</th><th>Url</th><th>Env</th><th>Last Ok Ping</th><th>LastError</th><th>Last ping duration</th></tr></thead>' +
            '<tbody>';
        var tableBottop = '</tbody></table>';
        var ok = "";
        var servicesCount = 0;
        for (var _i = 0, _a = Object.keys(status.services); _i < _a.length; _i++) {
            var key = _a[_i];
            var services = status.services[key];
            ok += '<tr><td colspan="8"><h3>' + key + '</h3></td>' +
                '</tr>';
            for (var _b = 0, services_1 = services; _b < services_1.length; _b++) {
                var service = services_1[_b];
                var started = "???";
                if (service.started) {
                    started = new Date(service.started / 1000).toLocaleString();
                }
                if (service.lastOk >= 5) {
                    ok += '<tr style="background:red">' +
                        '<td>' + service.id + '</td>' +
                        '<td>' + service.name + '</td>' +
                        '<td>' + service.version + '</td>' +
                        '<td><div>' + service.url + '</div><div> Started: ' + started + '</div></td>' +
                        '<td>' + service.envInfo + '</td>' +
                        '<td>' + service.lastOk + ' sec ago</td>' +
                        '<td>' + service.lastError + '</td>' +
                        '<td>' + service.lastPingDuration + '</td>' +
                        '</tr>';
                }
                else {
                    ok += '<tr>' +
                        '<td>' + service.id + '</td>' +
                        '<td>' + service.name + '</td>' +
                        '<td>' + service.version + '</td>' +
                        '<td><div>' + service.url + '</div><div> Started: ' + started + '</div></td>' +
                        '<td>' + service.envInfo + '</td>' +
                        '<td>' + service.lastOk + ' sec ago</td>' +
                        '<td>' + service.lastError + '</td>' +
                        '<td>' + service.lastPingDuration + '</td>' +
                        '</tr>';
                }
                servicesCount++;
            }
        }
        HtmlStatusBar.updateServicesAmount(servicesCount);
        return tableTop + ok + tableBottop;
    };
    return HtmlMain;
}());
//# sourceMappingURL=HtmlMain.js.map