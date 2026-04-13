# Bridle UI

This project is the Angular-based visual dashboard for `bridle-ctl`.

## 🚀 Technology Stack

- **Framework:** Angular v18+ (Standalone Components, Strict Typing)
- **State Management:** NgRx SignalStore
- **Styling:** Vanilla CSS (GitHub Primer aesthetics)
- **E2E Testing:** Playwright
- **Unit Testing:** Karma/Jasmine

## 🛠️ Development server

Run `npm run start` (or `ng serve`) for a dev server. Navigate to `http://localhost:4200/`. The application will automatically reload if you change any of the source files.

## 🏗️ Build

Run `npm run build` (or `ng build`) to build the project. The build artifacts will be stored in the `dist/` directory.

## 🧪 Testing

### Unit Tests

Run `npm run test` to execute the unit tests via [Karma](https://karma-runner.github.io).

### End-to-End Tests

Run `npx playwright test` to execute the end-to-end tests.

- View report: `npx playwright show-report`
- Run UI mode: `npx playwright test --ui`

## ✨ Code Quality & Best Practices

- **Strict TypeScript:** No `any`, `unknown`, or `never` bypasses.
- **Modern Angular:**
  - Uses `inject()` exclusively instead of constructors.
  - Utilizes new `input()` and `output()` signals.
  - Components enforce `ChangeDetectionStrategy.OnPush`.
- **Formatting & Linting:** Run `npm run lint` to catch stylistic and logical errors.
