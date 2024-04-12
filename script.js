console.log("Link js OK");

window.addEventListener('load', function() {
    var headerHeight = document.getElementsByClassName('header')[0].offsetHeight;
    document.body.style.paddingTop = headerHeight + 'px';
});

function switch_page_language() {
    let link = window.location.href.split("/").pop();
    if (link === "index.html" || link === "") {
    window.location.href = "index_en.html";
    }
    else {
        window.location.href = "index.html";
    }
}