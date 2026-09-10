import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { ThemeProvider } from "@/context/ThemeProvider";
import Home from "@/app/page";
import { expectNoSeriousViolations } from "@/test/axe";

/**
 * Automated accessibility gate for the landing page (the primary "key screen").
 *
 * Runs axe-core over the fully rendered page and fails the test if any
 * serious/critical violations are found. It also asserts the structural audit
 * dimensions (landmarks, heading hierarchy, labelled navigation) directly so a
 * regression is caught even where axe is lenient in jsdom.
 */
function renderLanding() {
  return render(
    <ThemeProvider>
      <Home />
    </ThemeProvider>,
  );
}

describe("Landing page accessibility", () => {
  it("reports no serious axe violations on the rendered page", async () => {
    const { container } = renderLanding();
    await expectNoSeriousViolations(container);
  });

  it("exposes a single top-level heading (h1)", () => {
    renderLanding();
    const h1s = screen.getAllByRole("heading", { level: 1 });
    expect(h1s).toHaveLength(1);
    expect(h1s[0]).toHaveTextContent(/decentralized savings/i);
  });

  it("uses a consistent heading hierarchy without skipping levels", () => {
    renderLanding();
    const headings = screen
      .getAllByRole("heading")
      .map((h) => Number(h.tagName.slice(1)));
    for (let i = 0; i < headings.length - 1; i++) {
      // A level may stay the same or increase by one; it must never skip up.
      expect(headings[i + 1] - headings[i]).toBeLessThanOrEqual(1);
    }
  });

  it("contains exactly one main landmark", () => {
    renderLanding();
    expect(screen.getAllByRole("main")).toHaveLength(1);
  });

  it("provides a labelled primary navigation landmark", () => {
    renderLanding();
    const navs = screen.getAllByRole("navigation");
    expect(navs.length).toBeGreaterThan(0);
    expect(navs[0]).toHaveAttribute("aria-label", "Primary");
  });

  it("wraps every interactive form control with a programmatic label", () => {
    renderLanding();
    // Every form input must be referenceable by an associated label.
    const inputs = screen.getAllByRole("textbox", { hidden: false });
    for (const input of inputs) {
      expect(input).toHaveAccessibleName();
    }
  });
});
