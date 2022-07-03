class Dialog {
    static prevHtml: string;

    public static show(html: string) {

        let el = document.getElementById('service-overview');

        this.prevHtml = el.innerHTML;

        el.innerHTML = '<div><button class="btn btn-light" onclick="Dialog.pressBack()">Back</button></div>' + html;
    }

    public static pressBack() {
        let el = document.getElementById('service-overview');
        el.innerHTML = this.prevHtml;
    }

}