$(document).ready(function() {
  function initWebsocket() {
    console.log("initWebsocket")
    path = "";
    if (location.protocol == "http:") {
      path += "ws://"
    } else {
      path += "wss://"
    }
    path += location.hostname + ":" + location.port + "/websocket"
    var ws = new ReconnectingWebSocket(path);

    ws.onopen = function () {
      console.log('Connection opened');
      $('#websocket').css("display", "block");
      $('#websocketStatus').removeClass('glyphicon-remove').addClass('glyphicon-ok');

      var articleName = $('head').data('article-dst-filename');
      console.log("Registering: ", articleName)
      ws.send(articleName)
    };

    ws.onmessage = function (msg) {
      //console.log (msg.data)
      var  dd = new diffDOM.DiffDOM();
      var htmlString = JSON.parse(msg.data)
      //console.log (htmlString)
      var outDiv = document.getElementById('NavAndArticle');
      var newElement = document.createElement("div");
      newElement.setAttribute("id", "NavAndArticle");
      newElement.innerHTML = htmlString;
      var diff = dd.diff(outDiv, newElement);
      //console.log(diff)
      dd.apply(outDiv, diff);

    };

    ws.onclose = function (msg) {
      $('#websocketStatus').removeClass('glyphicon-ok').addClass('glyphicon-remove');
      console.log('Connection closed');
    };
  };
  initWebsocket();
});