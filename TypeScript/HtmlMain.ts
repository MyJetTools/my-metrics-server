class HtmlMain {
    public static layout(): string {
        return '<div id="main"></div>' +
            HtmlStatusBar.layout();
    }



    public static generateServicesList(services: IServicesList): string {

        let result = '<table style="width:100%"><tr><td>';
        for (let service of services.services) {
            result += '<button type="button" class="btn btn-light" style="width:300px">' + service.id + '<div style="font-size:8px">' + this.micros_to_string(service.avg) + '<div></button>';
        }

        return result + '</td><td style="width:100%"></td></tr></table>';
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