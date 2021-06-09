import * as wasm from "rust-call-demo";

window.addEventListener('message', function(event) {
    var command = event.data.command,
        result = "invalid request";

    switch(command) {
    case 'render':
        result = wasm.test_return();
        break;
    }

    event.source.postMessage({'result': result}, event.origin);
});
