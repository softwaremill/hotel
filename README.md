# PoC for local-second, event-driven webapps

This repository implements a PoC for a simple hotel-management system. When online, it works as a traditional webapp, where users can create bookings, and hotel clerks can check guests in and out.

However, when offline, the app reverts to a degraded mode, allowing clerks to still check guests in (other operations are not available). The data is accumulated in events, which are then applied to the backend, when the app is back online. The backend also uses event sourcing, in a transactional variant, maintaining always-consistent projections.

The system consists of five components:
* a PostgreSQL database. In the setup, we assume you have one running locally. It needs to have the following configuration: `wal_level = 'logical'` and `max_replication_slots >= 1`.
* an ElectricSQL server, which connects to the locally running PostgreSQL database. It can be run using the provided `docker-compose.electric.yml` file.
* a Rust+Axum+sqlx backend, which exposes a REST API for the frontend, and also connects to the PostgreSQL database. It also proxies requests to the ElectricSQL server, to get real-time updates. The backend can be run from the `backend/` directory using `cargo run`.
* a TS+React frontend for users, allowing them to create bookings. It can be run from the `frontend-user/` directory using `npm run dev`.
* a TS+React frontend for hotel clerks, allowing them to see bookings in real-time, and check guests in and out. It can be run from the `frontend-front-desk/` directory using `npm run dev`.

Read more about the app and its architecture in the [Local-second, Event-Driven Web Apps](https://softwaremill.com/local-second-event-driven-webapps) article.