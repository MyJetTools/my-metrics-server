class HtmlMain {
    public static layout(): string {
        return '<div id="main"></div>' +
            HtmlStatusBar.layout();
    }



    public static generateServicesList(services: IServicesList): string {

        let result = '<table style="width:100%; height:100%"><tr style="vertical-align: top;"><td>';
        for (let service of services.services.sort((a, b) => b.avg > a.avg ? 1 : -1)) {
            result += '<button id="app-' + service.id + '" service="' + service.id + '" type="button" class="btn btn-light" style="width:300px" onclick="AppSelector.serviceSelected(this)">' + service.id + '<div style="font-size:8px">' + this.micros_to_string(service.avg) + '<div></button>';
        }

        return result + '</td><td id="service-overview" style="width:100%"></td></tr></table>';
    }


    public static generateServiceOverview(appId: string, overviews: IServiceOverviewList): string {

        let result = '<table class="table table-striped" style="font-size:10px"><tr><th>Data</th><th>Max</th><th>Min</th><th>Avg</th><th>Success</th><th>Errors</th><th>Total</th><th></th></tr>';
        for (let overview of overviews.data.sort((a, b) => b.avg > a.avg ? 1 : -1)) {
            let errors = overview.error.toFixed(0);

            if (overview.error > 0) {
                errors = '<span style="color:red">' + errors + '</span>';
            }
            result += '<tr><td>' + overview.data + '</td><td>' + this.micros_to_string(overview.max) + '</td><td>' + this.micros_to_string(overview.min) + '</td><td>' + this.micros_to_string(overview.avg) + '</td><td>' + overview.success + '</td><td>' + overview.error + '</td><td>' + overview.total + '</td><td>' +
                '<button data-app="' + appId + '" data-app-data="' + overview.data + '" class="btn btn-light btn-sm" onclick="AppSelector.expandMetrics(this)">Expand</button></td></tr>'
        }

        return result + '</table>';
    }


    public static generateMetrics(metrics: IMetrics): string {

        let result = '<table class="table table-striped" style="font-size:10px"><tr><th>Started</th><th>Duration</th><th>Message</th><th>Ip</th><th></th></tr>';
        for (let metric of metrics.metrics.sort((a, b) => a.started < b.started ? 1 : -1)) {

            let date = new Date(metric.started / 1000);

            let data = "";

            if (metric.success) {
                data = '<span style="color:green">' + metric.success + '</span>';
            }

            if (metric.error) {
                data = '<span style="color:red">' + metric.error + '</span>';
            }


            result += '<tr><td><div>' + date.toLocaleString() + '</div><div>' + date.toISOString() + '</div></td><td>' + this.micros_to_string(metric.duration) + '</td><td>' + data + '</td><td>' + metric.ip + '</td><td><button data-process-id="' + metric.id + '" class="btn btn-light btn-sm" onclick="AppSelector.showByProcessId(this)">Show</button></td></tr>'
        }

        return result + '</table>';
    }


    public static generateMetricsWithDuration(metrics: IMetricsByProcessId): string {
        if (metrics.metrics.length == 0) {
            return "No Data";
        }

        let { min, max } = this.getMaximumDuration(metrics);

        console.log("Min:" + min);
        console.log("Max:" + max);

        let maxDuration = max - min;
        console.log("MaxDur:" + maxDuration);


        let result = '<table class="table table-striped" style="font-size:10px"><tr><th>Started</th><th>Delayed</th><th>Name</th><th>Duration</th><th>Message</th><th>Ip</th><th>Delivery</br>Delay</th></tr>';
        let prevStarted: number;
        let prevEnded: number;
        for (let metric of metrics.metrics.sort((a, b) => a.started > b.started ? 1 : -1)) {

            let date = new Date(metric.started / 1000);

            let data = "";

            if (metric.success) {
                data = '<span style="color:green">' + metric.success + '</span>';
            }

            if (metric.error) {
                data = '<span style="color:red">' + metric.error + '</span>';
            }

            let pad = (metric.started - min) / maxDuration * 100;

            console.log("Pad:" + pad);

            let width = metric.duration / maxDuration * 100;

            console.log("Wid:" + width);

            let delayed = metric.started - min;

            let delayedStr = "";

            if (delayed > 0) {
                delayedStr = this.micros_to_string(delayed);
            }

            let prevDelayStr = "";

            if (prevStarted) {
                prevDelayStr = '<div>' + this.micros_to_string(metric.started - prevStarted) + '</div>';


            }

            let prevDeliveryDelayStr = "";
            if (prevEnded) {
                prevDeliveryDelayStr = '<div>' + this.micros_to_string(prevEnded - (metric.started + metric.duration)) + '</div>';
            }

            result += '<tr><td><div>' + date.toLocaleString() + '</div><div>' + date.toISOString() + '</div></td><td>' + delayedStr + prevDelayStr + '</td><td>' + metric.data + '</td><td>' + this.micros_to_string(metric.duration) + '</td><td>' + data + '</td><td>' + metric.ip + '</td><td>' + prevDeliveryDelayStr + '</td></tr>'
                + '<tr><td colspan="7"><span style="display: inline-block;margin-left:' + pad.toFixed(2) + '%;width:' + width.toFixed(2) + '%;height:5px; color: blue; background:blue;"></span></td></tr>';

            prevStarted = metric.started;
            prevEnded = metric.started + metric.duration;
        }

        return result + '</table>';
    }

    static getMaximumDuration(metrics: IMetricsByProcessId): { min: number, max: number } {
        let min = metrics.metrics[0].started;
        let max = metrics.metrics[0].started + metrics.metrics[0].duration;

        for (let metric of metrics.metrics) {
            if (min > metric.started) {
                min = metric.started;
            }

            let ended = metric.started + metric.duration;

            if (max < ended) {
                max = ended;
            }

        }

        return { min, max };
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