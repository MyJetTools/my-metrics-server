class HtmlMain {
    public static layout(): string {
        return '<div id="main"></div>' +
            HtmlStatusBar.layout();
    }


    public static generateContent(status: IStatusContract): string {
        let tableTop = '<table class="table table-striped">' +
            '<thead><tr><th>Id</th><th>Name</th><th>Version</th><th>Url</th><th>Env</th><th>Last Ok Ping</th><th>LastError</th><th>Last ping duration</th></tr></thead>' +
            '<tbody>';
        let tableBottop = '</tbody></table>'
        let ok = "";

        let servicesCount = 0


        for (let key of Object.keys(status.services)) {
            let services: IServiceStatus[] = status.services[key];
            ok += '<tr><td colspan="8"><h3>' + key + '</h3></td>' +
                '</tr>';

            for (var service of services) {
                let started = "???";

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
    }
}