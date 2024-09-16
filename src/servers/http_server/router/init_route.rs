//use serde_json::json;
use binuid_shared::hyper::{body::Incoming as IncomingBody, Request, Response};
use super::{BoxBody, full, Params};

pub(crate) async fn init_route(_req: Request<IncomingBody>, _params: Params) -> Response<BoxBody> {
    Response::new(full(
r#"<div id="app">Loading from wasm....</div>
<script type="module">
    import init, { binuid_engine } from '/binuid_engine';
    (async () => {
        await init();
        await binuid_engine();
    })();
</script>
<script type="text/javascript">
    window.onload = (event) => {
        let oldPushState = window.history.pushState;
        window.history.pushState = function pushState() {
            console.log("djed5");
            let ret = oldPushState.apply(this, arguments);
            window.dispatchEvent(new Event('pushstate'));
            window.dispatchEvent(new Event('locationchange'));
            return ret;
        };

        let oldReplaceState = window.history.replaceState;
        window.history.replaceState = function replaceState() {
            console.log("djed6");
            let ret = oldReplaceState.apply(this, arguments);
            window.dispatchEvent(new Event('replacestate'));
            window.dispatchEvent(new Event('locationchange'));
            return ret;
        };

        window.addEventListener('popstate', (event) => {
            event.preventDefault();
            console.log("djed2", event);
            window.dispatchEvent(new Event('locationchange'));
        });

        window.addEventListener('locationchange', (event) => {
            event.preventDefault();
            console.log("djed", event);
        });
    };
</script>
"#))
}
