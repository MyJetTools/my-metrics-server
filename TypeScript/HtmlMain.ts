class HtmlMain {
    public static layout(): string {
        return '<div id="main"></div>' +
            HtmlStatusBar.layout();
    }



    public static generateServicesList(services: IServicesList): string {

        let result = '<table style="width:100%"><tr><td>';
        for (let service of services.services) {
            result += '<button id="app-' + service.id + '" service="' + service.id + '" type="button" class="btn btn-light" style="width:300px" onclick="AppSelector.serviceSelected(this)">' + service.id + '<div style="font-size:8px">' + this.micros_to_string(service.avg) + '<div></button>';
        }

        return result + '</td><td id="service-overview" style="width:100%"></td></tr></table>';
    }


    public static generateServiceOverview(overviews: IServiceOverviewList): string {

        let result = '<table class="table table-striped" style="font-size:10px"><tr><th>Data</th><th>Max</th><th>Min</th><th>Avg</th><th>Success</th><th>Errors</th><th>Total</th></tr>';
        for (let overview of overviews.data) {
            let errors = overview.error.toFixed(0);

            if (overview.error > 0) {
                errors = '<span style="color:red">' + errors + '</span>';
            }
            result += '<tr><td>' + overview.data + '</td><td>' + this.micros_to_string(overview.max) + '</td><td>' + this.micros_to_string(overview.min) + '</td><td>' + this.micros_to_string(overview.avg) + '</td><td>' + overview.success + '</td><td>' + overview.error + '</td><td>' + overview.total + '</td></tr>'
        }

        return result + '</table>';
    }




    static micros_to_string(micros: number): string {
        if (micros < 1000) {
            return micros + 'micros';
        }

        if (micros < 1000000) {
            return (micros / 1000) + 'ms';
        }

        return (micros / 1000000) + 's';
    }

}