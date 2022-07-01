class HtmlMain {
    public static layout(): string {
        return '<div id="main"></div>' +
            HtmlStatusBar.layout();
    }



    public static generateServicesList(services: IServicesList): string {

        let result = '<table style="width:100%"><tr><td>';
        for (let service of services.names) {
            result += '<button type="button" class="btn btn-light" style="width:200px">' + service + '</button>';
        }

        return result + '</td><td style="width:100%"></td></tr></table>';
    }

}