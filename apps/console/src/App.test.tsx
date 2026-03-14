import { render, screen } from "@testing-library/react";
import { expect, test } from "vitest";
import App from "./App";

test("renders console heading", () => {
  render(<App />);
  expect(screen.getByText("EnvShield Console")).toBeTruthy();
});
