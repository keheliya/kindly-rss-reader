var scrollPosition = 0;
var bodyDiv = null;
var offsetNotScroll = null;
var darkTheme = false;
var zoom = 1.0;
var zoomStep = 0.1;

window.onload = function() {
    bodyDiv = document.getElementById("body");
    offsetNotScroll = Math.round(bodyDiv.offsetHeight * 0.1);
};


function scrollDown() {
    scrollPosition = Math.min(scrollPosition + bodyDiv.offsetHeight - offsetNotScroll, bodyDiv.scrollHeight);
    bodyDiv.scroll(0, scrollPosition);
}

function scrollUp() {
    scrollPosition = Math.max(scrollPosition - bodyDiv.offsetHeight + offsetNotScroll, 0);
    bodyDiv.scroll(0, scrollPosition);
}

function scrollToTop() {
    scrollPosition = 0;
    bodyDiv.scroll(0, scrollPosition);
}

function scrollToBottom() {
    scrollPosition = bodyDiv.scrollHeight - bodyDiv.offsetHeight;
    bodyDiv.scroll(0, scrollPosition);
}

function zoomIn() {
    zoom += zoomStep;
    bodyDiv.style.zoom = zoom;
}

function zoomOut() {
    zoom -= zoomStep;
    bodyDiv.style.zoom = zoom;
}

function toggleTheme() {
    document
      .querySelectorAll('link[rel=stylesheet].alternate')
      .forEach(function (node) { node.disabled = !node.disabled; });

    let toggleThemeButton = document.getElementById("toggleThemeButton");

    if (darkTheme) {
        toggleThemeButton.classList.remove("fa-moon-o");
        toggleThemeButton.classList.add("fa-sun-o");
    } else {
        toggleThemeButton.classList.remove("fa-sun-o");
        toggleThemeButton.classList.add("fa-moon-o");
    }

    darkTheme = !darkTheme;
}

(function onLoad() {
    let headerHeight = document.getElementById("header").offsetHeight;
    document.getElementById("body").style.height = `calc(100vh - ${headerHeight}px - 20px)`;
})()
