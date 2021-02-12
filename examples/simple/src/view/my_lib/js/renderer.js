

function page_init() {
    let d = new Cmd.data();
    d.insert("meta", "This text is from your javascript call!");
    d.select("meta", function (data) {
        const page = document.getElementById("page");
        page.innerHTML = "<p>" + data + "</p>";
    });
}