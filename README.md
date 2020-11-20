# Full Stack Rust - Percy Example App

> Percy Isomorphic App with Sled Database and GraphQL

This web app saves and loads data using [Percy](https://github.com/chinedufn/percy) for the UI, and [Sled](https://github.com/spacejam/sled), an embedded database, and GraphQL. All built in Rust. UI is compiled in WebAssembly, but serverside rendering also works.

## Scripts

- ./bootstrap.sh - Easy bootstrap script for freshly cloned repos on new machines
  - Note: Project uses submodules. They may be updated occasionally.
- ./start.sh - Build and run in development mode with Cargo
- ./build.release.sh - Optimized production build without debug symbols
- ./run.release.sh - Run the production build
  - Note: This script should not be used in production (use systemd script instead)
- ./clean.sh - Cleans all projects for debugging purposes
