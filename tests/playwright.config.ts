import { defineConfig, devices } from "@playwright/test";

// ═══════════════════════════════════════════════════════════════════════════════
// Playwright Configuration — Craole-CC
// ───────────────────────────────────────────────────────────────────────────────
//
// QUICK REFERENCE
//   pnpm test                  run all tests (local, reuses existing server)
//   pnpm test --ui             open interactive UI mode
//   pnpm test specs/theme      run a single spec module
//   pnpm test --debug          step through tests with Playwright inspector
//   CI=true pnpm test          simulate CI behaviour (fresh server, no retries waived)
//
// STRUCTURE
//   tests/
//   ├── fixtures/base.ts       shared typed fixtures & SEL constants
//   └── specs/
//       ├── foundation.spec.ts
//       ├── navigation.spec.ts
//       ├── theme.spec.ts
//       ├── hero.spec.ts
//       ├── footer.spec.ts
//       ├── accessibility.spec.ts
//       └── responsive.spec.ts
//
// ═══════════════════════════════════════════════════════════════════════════════

// ── Environment helpers ───────────────────────────────────────────────────────

const IS_CI = !!process.env.CI;

// ── Constants ─────────────────────────────────────────────────────────────────

/** The URL Playwright will use as the base for all relative `page.goto()` calls. */
const BASE_URL = process.env.BASE_URL ?? "http://localhost:3000";

/**
 * Per-test timeout (ms).
 * Leptos/WASM pages can be slower than typical JS apps on first paint, so we
 * give a little more headroom than the Playwright default (30 s).
 */
const TEST_TIMEOUT_MS = 45_000;

/**
 * Assertion timeout (ms).
 * How long `expect(locator).toBeVisible()` etc. will poll before failing.
 * Keep this well below TEST_TIMEOUT_MS so a single stuck assertion does not
 * silently eat the whole test budget.
 */
const EXPECT_TIMEOUT_MS = 8_000;

/**
 * Web-server startup timeout (ms).
 * The first `cargo leptos watch` compile (Rust + WASM) can take 60–90 s on a
 * cold cache. 3 minutes is generous enough for CI without being unbounded.
 */
const SERVER_TIMEOUT_MS = 180_000;

// ── Main config ───────────────────────────────────────────────────────────────

export default defineConfig({
  // ── Test discovery ──────────────────────────────────────────────────────────

  /** Root directory Playwright searches for spec files. */
  testDir: "./specs",

  /**
   * Glob pattern relative to `testDir`.
   * Explicitly targets the `specs/` subdirectory so the `fixtures/` folder
   * (which contains helpers, not tests) is never mistakenly executed.
   */
  // testMatch: "specs/**/*.spec.ts",

  // ── Timeouts ────────────────────────────────────────────────────────────────

  timeout: TEST_TIMEOUT_MS,
  expect: { timeout: EXPECT_TIMEOUT_MS },

  // ── Parallelism & reliability ────────────────────────────────────────────────

  /**
   * Run spec files in parallel.
   * Each spec gets its own browser context so tests remain fully isolated even
   * when running concurrently.
   */
  fullyParallel: true,

  /**
   * Prevent `test.only()` from silently skipping everything else on CI.
   * A leftover `.only` is always a mistake in a shared branch.
   */
  forbidOnly: IS_CI,

  /**
   * Retry flaky tests on CI only.
   * Locally, an immediate failure is more useful for fast feedback.
   */
  retries: IS_CI ? 2 : 0,

  /**
   * Single worker on CI avoids resource contention on constrained runners.
   * Locally, Playwright picks a sensible default based on CPU count.
   */
  workers: IS_CI ? 1 : undefined,

  // ── Reporting ───────────────────────────────────────────────────────────────

  /**
   * Reporters:
   *  - "list"  → concise per-test output in the terminal (always on).
   *  - "html"  → rich report written to `playwright-report/`; auto-opens
   *              after a failed local run.
   *
   * On CI the HTML report is still generated so it can be uploaded as an
   * artifact, but it will not auto-open.
   */
  reporter: IS_CI
    ? [["list"], ["html", { open: "never" }]]
    : [["list"], ["html", { open: "on-failure" }]],

  // ── Shared browser context defaults ─────────────────────────────────────────

  use: {
    /**
     * Absolute base for relative `page.goto('/path')` calls.
     * Override at runtime via the BASE_URL env var if you need to point at a
     * staging server: `BASE_URL=https://staging.example.com pnpm test`
     */
    baseURL: BASE_URL,

    /**
     * Capture a full Playwright trace on the first retry of any failing test.
     * Open with: `pnpm playwright show-trace trace.zip`
     * Set to "on" locally when debugging a hard-to-reproduce failure.
     */
    trace: "on-first-retry",

    /**
     * Capture a screenshot only when a test fails.
     * Screenshots land in `test-results/<test-name>/` alongside any traces.
     */
    screenshot: "only-on-failure",

    /**
     * Record a video only when a test fails (first retry on CI).
     * Videos are retention-friendly: they are not produced for passing tests.
     */
    video: "on-first-retry",
  },

  // ── Browser matrix ───────────────────────────────────────────────────────────

  projects: [
    // ── Desktop browsers (always active) ──────────────────────────────────────
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
    {
      name: "firefox",
      use: { ...devices["Desktop Firefox"] },
    },
    {
      name: "webkit",
      use: { ...devices["Desktop Safari"] },
    },
    // ── Mobile viewports ──────────────────────────────────────────────────────
    //
    // Uncomment to enable. The responsive.spec.ts suite already exercises
    // narrow viewports via the `mobilePage` fixture, but running against a
    // real mobile device profile catches additional rendering differences.
    //
    // { name: "Mobile Chrome", use: { ...devices["Pixel 5"] } },
    // { name: "Mobile Safari", use: { ...devices["iPhone 12"] } },

    // ── Branded browsers ──────────────────────────────────────────────────────
    //
    // Require the browser to be installed separately; not available on all CI
    // runners. Enable when you need to verify Edge/Chrome-specific behaviour.
    //
    // { name: "Microsoft Edge", use: { channel: "msedge" } },
    // { name: "Google Chrome",  use: { channel: "chrome" } },
  ],

  // ── Dev server ───────────────────────────────────────────────────────────────

  /**
   * Leptos dev-server lifecycle:
   *
   *  LOCAL  — `reuseExistingServer: true`
   *    If `cargo leptos watch` is already running on port 3000 (e.g. in a
   *    separate terminal), Playwright reuses it and skips the compile step,
   *    giving sub-second test startup.  If no server is found, it starts one.
   *
   *  CI     — `reuseExistingServer: false`
   *    Always starts a fresh server so the test environment is reproducible.
   *    The generous `timeout` accommodates a cold Rust + WASM compile on a
   *    runner without a warm cache.
   *
   *  stdout / stderr: "pipe"
   *    Suppresses the verbose cargo/leptos build output from the Playwright
   *    reporter.  Check the terminal running `cargo leptos watch` (or the CI
   *    job log) for build diagnostics.
   */
  webServer: {
    command: "cargo leptos watch",
    url: BASE_URL,
    reuseExistingServer: !IS_CI,
    timeout: SERVER_TIMEOUT_MS,
    stdout: "pipe",
    stderr: "pipe",
  },
});
