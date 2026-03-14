import { jsx as _jsx } from "react/jsx-runtime";
import { expect, test } from "vitest";
import { renderToString } from "react-dom/server";
import App from "./App";
test("renders console heading", () => {
    const output = renderToString(_jsx(App, {}));
    expect(output).toContain("EnvShield Console");
});
