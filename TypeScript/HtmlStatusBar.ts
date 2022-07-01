class HtmlStatusBar {

    private static servicesAmount: HtmlStaticElement<number>;

    private static connected: HtmlStaticElement<boolean>;

    static getConntected(): HtmlStaticElement<boolean> {
        if (this.connected === undefined) {
            this.connected = new HtmlStaticElement<boolean>(document.getElementById('connected'), (value) => value ? '<span style="color:green">Connected</span>' : '<span style="color:red">Disconnected</span>');
        }

        return this.connected;
    }

    static getServicesAmount(): HtmlStaticElement<number> {
        if (this.servicesAmount === undefined) {
            this.servicesAmount = new HtmlStaticElement<number>(document.getElementById('services-amount'), (value) => value.toFixed(0));
        }

        return this.servicesAmount;
    }



    public static layout(): string {
        return '<div id="status-bar">' +
            '<table><tr>' +


            '<td style="padding-left: 5px">Connected: <b id="connected" style="text-shadow: 0 0 2px white;"></b></td>' +
            '<td><div class="statusbar-separator"></div></td>' +

            '<td>Total Services: <b id="services-amount" style="text-shadow: 0 0 1px white;"></b></td>' +
            '<td><div class="statusbar-separator"></div></td>' +

            '</tr></table></div>';
    }

    public static updateServicesAmount(value: number) {
        this.getServicesAmount().update(value);
    }

    public static updateOffline() {
        this.getConntected().update(false);
    }
    public static updateOnline() {
        this.getConntected().update(true);
    }
}