console.log("Link js OK");

window.addEventListener('load', function() {
    var headerHeight = document.getElementsByClassName('header')[0].offsetHeight;
    document.body.style.paddingTop = headerHeight + 'px';
});

function switch_page_language() {
    if (window.location.href.split("/").pop() === "index.html") {
    window.location.href = "index_en.html";
    }
    else {
        window.location.href = "index.html";
    }
}