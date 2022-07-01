var HtmlStatusBar = /** @class */ (function () {
    function HtmlStatusBar() {
    }
    HtmlStatusBar.getConntected = function () {
        if (this.connected === undefined) {
            this.connected = new HtmlStaticElement(document.getElementById('connected'), function (value) { return value ? '<span style="color:green">Connected</span>' : '<span style="color:red">Disconnected</span>'; });
        }
        return this.connected;
    };
    HtmlStatusBar.getServicesAmount = function () {
        if (this.servicesAmount === undefined) {
            this.servicesAmount = new HtmlStaticElement(document.getElementById('services-amount'), function (value) { return value.toFixed(0); });
        }
        return this.servicesAmount;
    };
    HtmlStatusBar.layout = function () {
        return '<div id="status-bar">' +
            '<table><tr>' +
            '<td style="padding-left: 5px">Connected: <b id="connected" style="text-shadow: 0 0 2px white;"></b></td>' +
            '<td><div class="statusbar-separator"></div></td>' +
            '<td>Total Services: <b id="services-amount" style="text-shadow: 0 0 1px white;"></b></td>' +
            '<td><div class="statusbar-separator"></div></td>' +
            '</tr></table></div>';
    };
    HtmlStatusBar.updateServicesAmount = function (value) {
        this.getServicesAmount().update(value);
    };
    HtmlStatusBar.updateOffline = function () {
        this.getConntected().update(false);
    };
    HtmlStatusBar.updateOnline = function () {
        this.getConntected().update(true);
    };
    return HtmlStatusBar;
}());
//# sourceMappingURL=HtmlStatusBar.js.map