init();

async function init() {
    if (typeof __webpack_require__ !== "undefined") {
        // We run in the npm/webpack environment.
        const [{Chart}, {main, setup}] = await Promise.all([
            import("wasm-demo"),
            import("./index.js"),
        ]);
        setup(Chart);
        main();
    } else {
        // Direct browser load (no webpack). WASM is auto-initialized via __wbindgen_start.
        const [{Chart}, {main, setup}] = await Promise.all([
            import("../pkg/wasm_demo.js"),
            import("./index.js"),
        ]);
        setup(Chart);
        main();
    }
}
