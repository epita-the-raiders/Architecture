//console.log("Link js OK");

window.addEventListener('load', function() {
    var headerHeight = document.getElementsByClassName('header')[0].offsetHeight;
    document.body.style.paddingTop = headerHeight + 'px';
});

document.addEventListener('DOMContentLoaded', function() {
    const markdownContainer = document.querySelector('.md');
    if (markdownContainer) {
        const markdownContent = markdownContainer.textContent.trim();
        markdownContainer.innerHTML = marked.parse(markdownContent);
    }
    const openPopupDefense1 = document.getElementById('openPopupDefense1');
    const openPopupDefense2 = document.getElementById('openPopupDefense2');
    const openPopupPurpose = document.getElementById('openPopupPurpose');
    const popupDefense1 = document.getElementById('popupDefense1');
    const popupDefense2 = document.getElementById('popupDefense2');
    const popupPurpose = document.getElementById('popupPurpose');
    const closeButton = document.getElementById('closePopup');

    if (openPopupDefense1) {
        openPopupDefense1.addEventListener('click', function() {
            popupDefense1.style.display = 'block';
        });
    }

    if (openPopupDefense2) {
        openPopupDefense2.addEventListener('click', function() {
            popupDefense2.style.display = 'block';
        });
    }

    if (openPopupPurpose) {
        openPopupPurpose.addEventListener('click', function() {
            popupPurpose.style.display = 'block';
        });
    }

    if (popupDefense1) {
        popupDefense1.addEventListener('click', function(event) {
            if (event.target === popupDefense1) {
                popupDefense1.style.display = 'none';
            }
        });
    }

    if (popupDefense2) {
        popupDefense2.addEventListener('click', function(event) {
            if (event.target === popupDefense2) {
                popupDefense2.style.display = 'none';
            }
        });
    }

    if (popupPurpose) {
        popupPurpose.addEventListener('click', function(event) {
            if (event.target === popupPurpose) {
                popupPurpose.style.display = 'none';
            }
        });
    }

    if (closeButton) {
        closeButton.addEventListener('click', function() {
            popupDefense1.style.display = 'none';
            popupDefense2.style.display = 'none';
            popupPurpose.style.display = 'none';
        });
    }

    document.addEventListener('keydown', function(event) {
        if (event.key === 'Escape') {
            popupDefense1.style.display = 'none';
            popupDefense2.style.display = 'none';
            popupPurpose.style.display = 'none';
        }
    });

    let userAgent = window.navigator.userAgent;

    const linuxButton = document.querySelector('.linux');
    const windowsButton = document.querySelector('.windows');

    if (userAgent && linuxButton && windowsButton) {
        if (userAgent.indexOf("Win") != -1) {
            linuxButton.classList.remove('active');
            windowsButton.classList.add('active');
        }
        else if (userAgent.indexOf("Linux") != -1) {
            linuxButton.classList.add('active');
            windowsButton.classList.remove('active');
        }
    }

    if (linuxButton) {
        linuxButton.addEventListener('mouseover', function() {
            linuxButton.classList.add('active');
            windowsButton.classList.remove('active');
            linuxButton.classList.add('pulse');
        });

        linuxButton.addEventListener('mouseout', function() {
            linuxButton.classList.remove('pulse');
        });
    }

    if (windowsButton) {
        windowsButton.addEventListener('mouseover', function() {
            windowsButton.classList.add('active');
            linuxButton.classList.remove('active');
            windowsButton.classList.add('pulse');
        });

        windowsButton.addEventListener('mouseout', function() {
            windowsButton.classList.remove('pulse');
        });
    }

    var menuGui = document.querySelector('.menu-gui');
    if (menuGui) {
        menuGui.addEventListener('click', function() {
        var dropdownContentGui = this.querySelector('.dropdown-content-gui');
        dropdownContentGui.style.display = dropdownContentGui.style.display === 'block' ? 'none' : 'block';
        adjustTitleMargins('.menu-gui', dropdownContentGui);
        })
    }

    var menuAlgo = document.querySelector('.menu-algo');
    if (menuAlgo) {
        menuAlgo.addEventListener('click', function() {
        var dropdownContentAlgo = this.querySelector('.dropdown-content-algo');
        dropdownContentAlgo.style.display = dropdownContentAlgo.style.display === 'block' ? 'none' : 'block';
        adjustTitleMargins('.menu-algo', dropdownContentAlgo);
        })
    }

    var menuWebSite = document.querySelector('.menu-web');
    if (menuWebSite) {
        menuWebSite.addEventListener('click', function() {
        var dropdownContentWebSite = this.querySelector('.dropdown-content-web');
        dropdownContentWebSite.style.display = dropdownContentWebSite.style.display === 'block' ? 'none' : 'block';
        adjustTitleMargins('.menu-web', dropdownContentWebSite);
        })
    }

    function adjustTitleMargins(menuClass, dropdownContent) {
        var menuContainer = document.querySelector(menuClass);

        for (let i = 0; i < 500; i++) {
            setTimeout(function() {
                var dropdownHeight = window.getComputedStyle(dropdownContent).height;
        
                if (dropdownContent.style.display === 'block') {
                    menuContainer.style.marginBottom = parseInt(dropdownHeight.split('px')[0])+50 + 'px';
                } else {
                    menuContainer.style.marginBottom = '50px';
                }
            }, i);
        }
    }
});

function switch_page_language() {
    var docName = window.location.href.split("/").pop();
    if (docName.split("_").pop().split(".")[0] === "en") {
        window.location.href = docName.split(".")[0].split("_")[0] + ".html";
    }
    else {
        window.location.href = docName.split(".")[0] + "_en.html";
    }
}

function switch_index_page() {
    var page = "index";
    if (window.location.href.split("/").pop().split("_").pop().split(".")[0] === "en") {
        window.location.href = page + "_en.html";
    }
    else {
        window.location.href = page + ".html";
    }
}

function switch_biography_page() {
    var page = "biography";
    if (window.location.href.split("/").pop().split("_").pop().split(".")[0] === "en") {
        window.location.href = page + "_en.html";
    }
    else {
        window.location.href = page + ".html";
    }
}

function switch_download_page() {
    var page = "download";
    if (window.location.href.split("/").pop().split("_").pop().split(".")[0] === "en") {
        window.location.href = page + "_en.html";
    }
    else {
        window.location.href = page + ".html";
    }
}

function switch_bibliography_page() {
    var page = "bibliography";
    if (window.location.href.split("/").pop().split("_").pop().split(".")[0] === "en") {
        window.location.href = page + "_en.html";
    }
    else {
        window.location.href = page + ".html";
    }
}

function switch_appendices_page() {
    var page = "appendices";
    if (window.location.href.split("/").pop().split("_").pop().split(".")[0] === "en") {
        window.location.href = page + "_en.html";
    }
    else {
        window.location.href = page + ".html";
    }
}

function openTab(link) {
    window.open("https://" + link, "_blank");
}

function download(os) {
    if (window.location.href.split("/").pop().split("_").pop().split(".")[0] === "en") {
        if (os === "windows") {
            alert("The Windows version is not and will not be available for this presentation.");
        }
        else if (os === "linux") {
            openTab("www.mediafire.com/file/y8dk7qgrflg628s/RaidersArchitecture.tar.gz/file");
        }
    }
    else {
        if (os === "windows") {
            alert("La version Windows n'est et ne sera pas disponible pour cette présentation.");
        }
        else if (os === "linux") {
            openTab("www.mediafire.com/file/y8dk7qgrflg628s/RaidersArchitecture.tar.gz/file");
        }
    }
}