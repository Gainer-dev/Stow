import axe from "axe-core";

/**
 * Runs axe-core over `container` and returns only serious/critical violations.
 *
 * Note: color-contrast requires a real rendering engine with compositing and is
 * not determinable in jsdom, so axe reports it as "incomplete" rather than a
 * pass/fail there. We disable it here (standard practice for jsdom) and rely on
 * a manual palette audit (see globals.css tokens) for contrast instead.
 */
export interface AxeOptions {
  /** Extra axe rules to toggle on top of the defaults. */
  rules?: Record<string, { enabled: boolean }>;
  /** Restrict scanning to a list of rule ids. */
  runOnly?: string[];
}

export async function getAxeViolations(
  container: HTMLElement,
  { rules = {}, runOnly }: AxeOptions = {},
): Promise<axe.Result[]> {
  const options: axe.RunOptions = {
    runOnly: runOnly ? { type: "rule", values: runOnly } : undefined,
    rules: {
      // Requires real rendering; not evaluable in jsdom.
      "color-contrast": { enabled: false },
      ...rules,
    },
  };
  const result = await axe.run(container, options);
  return result.violations.filter(
    (v) => v.impact === "serious" || v.impact === "critical",
  );
}

/** Formats violations into a readable failure message for assertions. */
export function formatAxeViolations(violations: axe.Result[]): string {
  if (violations.length === 0) return "";
  return (
    "\nAxe found serious/critical accessibility violations:\n" +
    violations
      .map((v) => {
        const nodes = v.nodes
          .map((n) => `    - ${n.target.join(" ")} (${n.html})`)
          .join("\n");
        return `  [${v.impact}] ${v.id}: ${v.help}\n    ${v.helpUrl}\n${nodes}`;
      })
      .join("\n")
  );
}

/**
 * Asserts there are no serious/critical axe violations in `container`.
 * Call inside an async test, e.g. `await expectNoSeriousViolations(container)`.
 */
export async function expectNoSeriousViolations(
  container: HTMLElement,
  options?: AxeOptions,
): Promise<void> {
  const violations = await getAxeViolations(container, options);
  if (violations.length > 0) {
    throw new Error(formatAxeViolations(violations));
  }
}
