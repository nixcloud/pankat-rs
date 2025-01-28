var MetaData
function getURLParameter(name) {
    return decodeURIComponent((new RegExp('[?|&]' + name + '=' + '([^&;]+?)(&|#|;|$)').exec(location.search)||[,""])[1].replace(/\+/g, '%20'))||null
}

var toClearSelectionView = function() {
    setFilter("", 1);
    $("#FilterPreSelection").css('display', 'block');
    $("#FilterSelection").css('display', 'none');
    $("#FilterPostSelection").css('display', 'none');
    $("#timeline").css('display', 'block');
}

var toSelectionView = function() {
    $("#FilterPreSelection").css('display', 'none');
    $("#FilterSelection").css('display', 'block');
    $("#FilterPostSelection").css('display', 'none');
    $("#timeline").css('display', 'none');
}

var toPostSelectionView = function() {
    $("#FilterPreSelection").css('display', 'none');
    $("#FilterSelection").css('display', 'none');
    $("#FilterPostSelection").css('display', 'block');
    $("#timeline").css('display', 'block');
}


var setFilter = function(filter, addHistory) {
    var selection = []
    try {
        var type = filter.split("::")[0]
        var identifier = filter.split("::")[1]
    } catch(e) {
        console.log("removing filter selection because of split error handling")
        for (i=0; i < MetaData.ArticleCount; i++) {
            var n = ".posting_" + i;
            $(n).css('display', 'block');
        }
        if (addHistory === 1) {
            history.pushState('', '',  window.location.pathname);
        }
        return
    }
    // hide all years
    $(".pankat_year").css('display', 'none');
    if (type == "tag" && typeof(MetaData.Tags[identifier]) !== "undefined") {
        selection = MetaData.Tags[identifier]
    } else if (type == "series" && typeof(MetaData.Series[identifier]) !== "undefined") {
        selection = MetaData.Series[identifier]
    } else {
        console.log("removing filter selection")
        for (i=0; i < MetaData.ArticleCount; i++) {
            var n = ".posting_" + i;
            $(n).css('display', 'block');
        }
        $(".pankat_year").css('display', 'block');
        if (addHistory === 1) {
            history.pushState('', '',  window.location.pathname);
        }
        return
    }

    //console.log(selection, type, identifier)

    for (i=0; i < MetaData.ArticleCount; i++) {
        var n = ".posting_" + i;
        if (selection.includes(i)) {
            //console.log("show", n)
            $(n).css('display', 'block');
            // show respective year
            Object.keys(MetaData.Years).forEach(function(key, index) {
                MetaData.Years[key].forEach(function(article) {
                    if (article === i) {
                        year = parseInt(key);
                        var n = ".pankat_year_" + year;
                        console.log("Showing year with jquery class: ",  n)

                        $(n).css('display', 'block');
                        var n = ".pankat_year_" + (year + 1);
                        $(n).css('display', 'block');
                        console.log("Showing year with jquery class: ",  n)

                    }
                })
            });
        } else {
            //console.log("hide", n)
            $(n).css('display', 'none');
        }
    }

    if (addHistory === 1) {
        history.pushState('', '',  window.location.pathname + '?filter=' + filter);
    }

    // set inner html of filter selection
    var filterSelection = document.getElementById("FilterSelectionText");
    filterSelection.innerHTML = '';
    if (type == "tag" && typeof(MetaData.Tags[identifier]) !== "undefined") {
        filterSelection.innerHTML = 'Your selection: <a class="tagbtn btn btn-primary"">' + identifier + '</a>'
    }
    if (type == "series" && typeof(MetaData.Series[identifier]) !== "undefined") {
        filterSelection.innerHTML = 'Your selection: <a class="seriesbtn btn btn-primary"">' + identifier + '</a>'
    }
    toPostSelectionView();
}

$(document).ready(function() {
    MetaData = JSON.parse(document.getElementById('MetaData', 0).innerHTML)
    var filter = getURLParameter("filter");
    //console.log("document.ready(), filter: " + filter)
    $.timeliner({
        oneOpen: false,
        startState: 'open'
    });
    setFilter(filter, 0)
});

// browser pageContent button was used, so we need to update the posts, but not the browser pageContent
window.addEventListener("popstate", function() {
    var filter = getURLParameter("filter");
    setFilter(filter, 0);
});