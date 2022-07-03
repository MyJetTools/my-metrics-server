var AppSelector = /** @class */ (function () {
    function AppSelector() {
    }
    AppSelector.serviceSelected = function (el) {
        var _this = this;
        if (this.requested) {
            return;
        }
        this.requested = true;
        if (this.selectedApp) {
            var prevBtn = document.getElementById('app-' + this.selectedApp);
            prevBtn.classList.remove('btn-primary');
            prevBtn.classList.add('btn-light');
        }
        var appId = el.getAttribute('service');
        var currentBtn = document.getElementById('app-' + appId);
        currentBtn.classList.remove('btn-light');
        currentBtn.classList.add('btn-primary');
        this.selectedApp = appId;
        $.ajax({ url: '/ui/GetServiceOverview?id=' + appId, type: 'get', })
            .then(function (result) {
            _this.requested = false;
            var el = document.getElementById('service-overview');
            el.innerHTML = HtmlMain.generateServiceOverview(appId, result);
            HtmlStatusBar.updateOnline();
        }).fail(function () {
            _this.requested = false;
            HtmlStatusBar.updateOffline();
        });
    };
    AppSelector.expandMetrics = function (el) {
        var _this = this;
        if (this.requested) {
            return;
        }
        this.requested = true;
        var request = {
            id: el.getAttribute('data-app'),
            data: el.getAttribute('data-app-data'),
        };
        var str = jQuery.param(request);
        $.ajax({ url: '/ui/GetByServiceData?' + str, type: 'get', })
            .then(function (result) {
            _this.requested = false;
            Dialog.show(HtmlMain.generateMetrics(result));
            HtmlStatusBar.updateOnline();
        }).fail(function () {
            _this.requested = false;
            HtmlStatusBar.updateOffline();
        });
    };
    AppSelector.requested = false;
    return AppSelector;
}());
//# sourceMappingURL=AppSelector.js.map