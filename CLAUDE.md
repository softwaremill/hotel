# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

The app is a demo of a simple hotel room booking system.

- `backend/` - Backend services, exposing an HTTP API
- `frontend-front-desk/` - Front desk staff interface 
- `frontend-user/` - Guest/customer interface

## Development Setup

The project appears to be in early stages with empty directories. Based on the `.gitignore` file, this project is set up for Rust development with Cargo as the build system.

## Project Structure

```
hotel/
├── backend/           # Backend services
├── frontend-front-desk/  # Front desk interface
├── frontend-user/     # Guest interface
├── .vscode/          # VS Code settings
├── .gitignore        # Git ignore rules for Rust
└── LICENSE           # Project license
```

## Next Steps

This repository appears to be newly initialized. When development begins:

1. Add `Cargo.toml` files to define Rust projects in each directory
2. Implement the backend API services
3. Build the frontend applications (likely web-based)
4. Add build scripts and development commands

## Notes

- Project uses Rust toolchain (evidenced by gitignore patterns)
- Multi-frontend architecture suggests different user roles (staff vs guests)
- Repository is currently at initial commit stage