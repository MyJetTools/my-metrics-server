
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

                el.innerHTML = HtmlMain.generateServiceOverview(result);


                HtmlStatusBar.updateOnline();

            }).fail(() => {
                this.requested = false;
                HtmlStatusBar.updateOffline();
            })

    }
}