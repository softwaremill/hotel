# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

The app is a hotel room booking system with event sourcing architecture.

- `backend/` - Rust backend with Axum web framework, PostgreSQL database, event sourcing
- `frontend-front-desk/` - React TypeScript front desk staff interface (Vite)
- `frontend-user/` - React TypeScript guest interface (Vite, currently minimal)

## Development Commands

### Backend (Rust)
- `cd backend && cargo run` - Run the backend server (port 3000)
- `cd backend && cargo check` - Check code without building
- `cd backend && cargo build` - Build the project
- `cd backend && cargo test` - Run tests

### Frontend Front Desk
- `cd frontend-front-desk && npm run dev` - Development server
- `cd frontend-front-desk && npm run build` - Production build (TypeScript compile + Vite build)
- `cd frontend-front-desk && npm run lint` - ESLint checking
- `cd frontend-front-desk && npm run preview` - Preview production build

### Frontend User
- `cd frontend-user && npm run dev` - Development server  
- `cd frontend-user && npm run build` - Production build
- `cd frontend-user && npm run lint` - ESLint checking

## Architecture Overview

### Backend Architecture
- **Event Sourcing**: Uses event store pattern with PostgreSQL
- **Event Types**: `BookingCreated`, `BookingCheckedIn` events in `models_events.rs`
- **Projections**: Event handlers update read models in `projections.rs`
- **Database**: PostgreSQL with SQLx for migrations and queries
- **API**: REST endpoints for hotels, bookings, and check-in operations
- **Room Assignment**: Automatic room assignment logic in `room_assignment.rs`

### Frontend Architecture  
- **Front Desk**: Multi-page React app with React Router
  - Hotel selection page
  - Bookings dashboard with date filtering and check-in functionality
- **State Management**: Component-level state, no global state management
- **API Integration**: Fetch calls to backend REST API
- **Styling**: CSS modules/plain CSS

### Database Schema
- `events` - Event store table for all domain events
- `hotels` - Hotel master data
- `bookings` - Projection table for current booking state
- Migrations in `backend/migrations/` directory

### Key Domain Models
- `Event` enum with `BookingCreated` and `BookingCheckedIn` variants
- `BookingStatus` enum: `Confirmed`, `CheckedIn`
- Hotels have rooms numbered 1-N, automatically assigned on check-in