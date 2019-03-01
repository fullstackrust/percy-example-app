# InnoTrade Contractor

Cleansheet InnoTrade Contractor App built in fullstack Rust

## Scripts

- ./bootstrap.sh - Easy bootstrap script for freshly cloned repos on new machines
    - Note: Project uses submodules. They may be updated occasionally.
- ./start.sh - Build and run in development mode with Cargo
- ./build.release.sh - Optimized production build without debug symbols
- ./run.release.sh - Run the production build
    - Note: This script should not be used in production (use systemd script instead)
- ./clean.sh - Cleans all projects for debugging purposes
