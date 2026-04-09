@echo off
SETLOCAL

if "%~1"=="" goto help
if /I "%~1"=="install_base" goto install_base
if /I "%~1"=="install_deps" goto install_deps
if /I "%~1"=="docs_rust" goto docs_rust
if /I "%~1"=="docs_angular" goto docs_angular
if /I "%~1"=="build" goto build
if /I "%~1"=="test" goto test
if /I "%~1"=="docs" goto docs

:help
echo Available targets: 
echo   install_base   - Install language runtime and base dependencies (Winget)
echo   install_deps   - Install local dependencies (Cargo, npm, MkDocs)
echo   docs_rust      - Generate Rust documentation
echo   docs_angular   - Generate Angular documentation
echo   build          - Build the Rust and Angular projects
echo   test           - Run tests for Rust and Angular projects
echo   docs           - Build the unified Material for MkDocs site
goto end

:install_base
echo Installing base dependencies (Windows)...
echo Ensuring Winget is available...
winget install -e --id Rustlang.Rustup
winget install -e --id OpenJS.NodeJS
winget install -e --id Python.Python.3.11
winget install -e --id Kitware.CMake
goto end

:install_deps
echo Installing project dependencies...
cargo fetch
cd bridle-ui
call npm install
cd ..
python -m pip install --upgrade pip mkdocs-material
goto end

:docs_rust
echo Generating Rust docs...
cargo doc --no-deps --document-private-items
goto end

:docs_angular
echo Generating Angular docs...
cd bridle-ui
call npx compodoc -p tsconfig.doc.json
cd ..
goto end

:build
echo Building projects...
cargo build
cd bridle-ui
call npm run build
cd ..
goto end

:test
echo Running tests...
cargo test
cd bridle-ui
call npm test -- --watch=false
cd ..
goto end

:docs
call :docs_rust
call :docs_angular
echo Building MkDocs site...
python -m mkdocs build

echo Merging Rust docs into MkDocs site...
if not exist site\rust mkdir site\rust
xcopy /E /I /Y target\doc\* site\rust\ >nul

echo Merging Angular docs into MkDocs site...
if not exist site\angular mkdir site\angular
xcopy /E /I /Y bridle-ui\docs\* site\angular\ >nul

echo [OK] Docs successfully generated in .\site
goto end

:end
ENDLOCAL
