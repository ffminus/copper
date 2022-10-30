// Fetch WASM module
importScripts("/pkg/copper.js");

// Initialize WASM module
(async () => await wasm_bindgen("/pkg/copper_bg.wasm"))();

// Import class defined in module
const { Model } = wasm_bindgen;

onmessage = ({ data: { weights, values, weightMax } }) => {
    const m = new Model();

    const xs = m.newVarsBinary(weights.length);

    const weight = m.linear(xs, weights);

    m.leq(weight, m.cst(weightMax));

    const value = m.linear(xs, values);

    const solution = m.maximize(value);

    // Problem might have no feasible solution (negative maximum weight)
    const data = solution === undefined ? undefined : {
        weight: solution.getValue(weight),
        value: solution.getValue(value),
        items: solution.getValues(xs),
    };

    postMessage(data);
};
