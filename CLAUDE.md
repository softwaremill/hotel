# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Hotel booking system with real-time synchronization using Electric SQL. Features event sourcing architecture with PostgreSQL and real-time data sync between multiple frontend instances.

- `backend/` - Rust backend with Axum, PostgreSQL, event sourcing, Electric SQL integration
- `frontend-front-desk/` - React TypeScript frontend for hotel staff with real-time bookings
- `frontend-user/` - Guest interface (basic structure)
- `docker-compose.electric.yml` - Electric SQL container configuration

## Development Commands

### Backend (Rust)
- `cd backend && cargo run` - Run the backend server (port 3000)
- `cd backend && cargo check` - Type checking
- `cd backend && cargo build` - Build the project

### Frontend Front Desk
- `cd frontend-front-desk && npm run dev` - Development server
- `cd frontend-front-desk && npm run build` - Production build (TypeScript compile + Vite build)
- `cd frontend-front-desk && npm run lint` - ESLint checking

### Electric SQL
- `docker compose -f docker-compose.electric.yml up -d` - Start Electric container
- `docker compose -f docker-compose.electric.yml logs -f electric` - View Electric logs
- `docker compose -f docker-compose.electric.yml down` - Stop Electric container

## Architecture Overview

### Backend Architecture
- **Event Sourcing**: PostgreSQL-based event store with event replay
- **Real-time Sync**: Electric SQL proxy for streaming database changes
- **Event Types**: `BookingCreated`, `BookingCheckedIn`, `BookingCheckedOut`, `BookingCancelled` in `models_events.rs`
- **Projections**: Event handlers update read models in `projections.rs`
- **Electric Proxy**: Streaming HTTP proxy in `electric_proxy.rs` with server-side filtering
- **Room Assignment**: Automatic room assignment logic in `room_assignment.rs`

### Frontend Architecture  
- **Real-time Updates**: Electric SQL React hooks (`@electric-sql/react`)
- **Shape Subscriptions**: Server-filtered real-time booking data streams
- **No Polling**: Electric handles automatic UI updates when backend changes occur
- **Multi-page**: Hotel selection + bookings dashboard with React Router

### Database Requirements
- **PostgreSQL 17+** with logical replication enabled (`wal_level=logical`)
- **Connection**: `postgresql://postgres:postgres@localhost:5432/hotel`
- **Electric Integration**: Uses logical replication slot for real-time sync

### Key API Endpoints
- `GET /hotels/{id}/bookings/shape?date=YYYY-MM-DD` - Real-time bookings stream (mandatory date filter)
- `POST /bookings/{id}/checkin?today=YYYY-MM-DD` - Check in booking with room assignment
- `POST /bookings/{id}/checkout` - Check out booking
- `POST /bookings/{id}/cancel` - Cancel booking

### Domain Models
- `BookingStatus` enum: `Confirmed`, `CheckedIn`, `CheckedOut`, `Cancelled`
- Event-driven state transitions with automatic room assignment
- Server-side date filtering for efficient real-time subscriptions