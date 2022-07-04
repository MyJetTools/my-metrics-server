
class AppSelector {

    private static selectedApp: string;

    private static requested = false;

    public static serviceSelected(el: HTMLObjectElement) {

        if (this.requested) {
            return;
        }

        this.requested = true;



        if (this.selectedApp) {
            let prevBtn = document.getElementById('app-' + this.selectedApp);
            prevBtn.classList.remove('btn-primary');
            prevBtn.classList.add('btn-light');
        }

        let appId = el.getAttribute('service');

        let currentBtn = document.getElementById('app-' + appId);
        currentBtn.classList.remove('btn-light');
        currentBtn.classList.add('btn-primary');
        this.selectedApp = appId;


        $.ajax({ url: '/ui/GetServiceOverview?id=' + appId, type: 'get', })
            .then((result: IServiceOverviewList) => {
                this.requested = false;
                let el = document.getElementById('service-overview');

                el.innerHTML = HtmlMain.generateServiceOverview(appId, result);


                HtmlStatusBar.updateOnline();

            }).fail(() => {
                this.requested = false;
                HtmlStatusBar.updateOffline();
            })

    }


    public static expandMetrics(el: HTMLElement) {
        if (this.requested) {
            return;
        }

        this.requested = true;

        let request = {
            id: el.getAttribute('data-app'),
            data: el.getAttribute('data-app-data'),
        }

        var str = jQuery.param(request);


        $.ajax({ url: '/ui/GetByServiceData?' + str, type: 'get', })
            .then((result: IMetrics) => {
                this.requested = false;
                Dialog.show(request.id + ':' + request.data, HtmlMain.generateMetrics(result))
                HtmlStatusBar.updateOnline();

            }).fail(() => {
                this.requested = false;
                HtmlStatusBar.updateOffline();
            })

    }

    public static showByProcessId(el: HTMLElement) {
        if (this.requested) {
            return;
        }

        this.requested = true;

        let processId = el.getAttribute('data-process-id');



        $.ajax({ url: '/ui/GetByProcessId?processId=' + processId, type: 'get', })
            .then((result: IMetricsByProcessId) => {
                this.requested = false;
                Dialog.override(processId, HtmlMain.generateMetricsWithDuration(result))
                HtmlStatusBar.updateOnline();

            }).fail(() => {
                this.requested = false;
                HtmlStatusBar.updateOffline();
            })

    }
}