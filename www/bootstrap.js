init();

async function init() {
    if (typeof process == "object") {
        // We run in the npm/webpack environment.
        const [{}, {main}] = await Promise.all([
            import("../pkg/BacteriaGraphing.js"),
            import("./index.js"),
        ]);
        main();
    } else {
        const [{default: init}, {main}] = await Promise.all([
            import("../pkg/BacteriaGraphing.js"),
            import("./index.js"),
        ]);
        await init();
        main();
    }
}
