import rust from "@wasm-tool/rollup-plugin-rust";

export default {
    input: {
        "userscripts-filmow": "../Cargo.toml",
    },
    output: {
        dir: "dist",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            inlineWasm: true,
        }),
    ],
};
