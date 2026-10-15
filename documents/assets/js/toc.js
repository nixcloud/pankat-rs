// static table of contents (pandoc-lua/toc.lua): copies it from the article into #toc and, like jquery.tocify,
// highlights the section in view and opens its subsections. initToc() runs again after a live update (diffDOM)
(function () {
  // the heading counts as in view from this distance to the top of the window on (tocify's highlightOffset)
  var offset = 40;
  var toc = null;
  var entries = []; // { heading, li } in document order

  function update() {
    if (!toc || entries.length === 0) {
      return;
    }
    var current = entries[0];
    var atBottom = window.innerHeight + window.scrollY >= document.documentElement.scrollHeight - 2;
    for (var i = 0; i < entries.length; i++) {
      if (atBottom || entries[i].heading.getBoundingClientRect().top <= offset) {
        current = entries[i];
      }
    }
    toc.querySelectorAll("li.active, li.open").forEach(function (li) {
      li.classList.remove("active", "open");
    });
    current.li.classList.add("active");
    for (var node = current.li; node && node !== toc; node = node.parentElement) {
      if (node.tagName === "LI") {
        node.classList.add("open");
      }
    }
  }

  window.initToc = function () {
    toc = document.getElementById("toc");
    var source = document.getElementById("toc-source");
    if (!toc) {
      return;
    }
    toc.innerHTML = source ? source.innerHTML : "";
    entries = [];
    toc.querySelectorAll('a[href^="#"]').forEach(function (a) {
      var heading = document.getElementById(decodeURIComponent(a.getAttribute("href").slice(1)));
      if (heading) {
        entries.push({ heading: heading, li: a.parentElement });
      }
    });
    update();
  };

  var scheduled = false;
  window.addEventListener(
    "scroll",
    function () {
      if (!scheduled) {
        scheduled = true;
        window.requestAnimationFrame(function () {
          scheduled = false;
          update();
        });
      }
    },
    { passive: true },
  );
  document.addEventListener("DOMContentLoaded", window.initToc);
})();
