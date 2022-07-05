class Dialog {
    static prevHtml: string;

    public static show(header: string, html: string) {

        let el = document.getElementById('service-overview');


        window.history.pushState({ div: 'service-overview', content: el.innerHTML }, "Details", '#service-overview');

        el.innerHTML = '<div><table><tr><td><button class="btn btn-light" onclick="Dialog.pressBack()">Back</button></td><td><h4>' + header + '</h4></td></table></div>' + html;
    }



    public static pressBack() {
        window.history.back()
    }

}