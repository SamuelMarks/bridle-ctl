# Bridle UI

Welcome to **Bridle UI**, the Angular-based visual dashboard for `bridle-ctl`.

While the CLI and Agent operate primarily in the terminal and background, the UI provides a rich, real-time interface for administrators to monitor AI agents, view database metrics, manage batch pipelines, and review local Pull Requests before they are synced upstream.

## 🚀 Technology Stack

- **Framework:** Angular v18+ (Standalone Components, Strict Typing)
- **State Management:** NgRx SignalStore (for highly reactive, boilerplate-free state)
- **Styling:** Vanilla CSS (GitHub Primer aesthetics for a familiar developer experience)
- **E2E Testing:** Playwright
- **Unit Testing:** Karma / Jasmine

## 🛠️ Getting Started

### Development Server

Run the following command to start the development server:
```bash
npm install
npm run start
```
Navigate to `http://localhost:4200/`. The application will automatically reload if you change any of the source files. 

*(Note: The UI expects the `bridle-rest` API to be running on port 8080. Start it via `bridle rest --port 8080` in a separate terminal).*

### Build for Production

Run `npm run build` to build the project. The optimized build artifacts will be stored in the `dist/` directory, ready to be served by any static file server or bundled into the Rust binary.

## 🧪 Testing and Quality Assurance

We maintain strict quality standards to ensure the UI is as reliable as the Rust backend.

### Unit Tests
Run unit tests via Karma:
```bash
npm run test
```

### End-to-End Tests
We use Playwright to ensure the entire user journey (from creating a pipeline to approving a PR) functions correctly.
```bash
# Run headless tests
npx playwright test

# View HTML report
npx playwright show-report

# Run tests in interactive UI mode
npx playwright test --ui
```

## ✨ Code Quality & Best Practices

When contributing to `bridle-ui`, please adhere to the following:

- **Strict TypeScript:** No `any`, `unknown`, or `never` bypasses. Define strict interfaces for all API payloads.
- **Modern Angular Features:**
  - Use `inject()` exclusively instead of constructor injection.
  - Utilize the new `input()` and `output()` signal APIs.
  - All components MUST enforce `ChangeDetectionStrategy.OnPush`.
- **Formatting & Linting:** Run `npm run lint` before committing to catch stylistic and logical errors.
- **CSS:** Avoid heavy frameworks. We use Vanilla CSS styled to match the standard Git Forge experience.