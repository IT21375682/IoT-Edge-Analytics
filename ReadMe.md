
# VisionTrack - IoT Telemetry Processing System

**VisionTrack** is a system designed for processing telemetry data from IoT devices, integrating multiple technologies such as **Django**, **Rust**, **Celery**, **Redis**, **Postgres**, and **Prometheus/Grafana**. It allows efficient management and visualization of real-time telemetry and IoT data using a scalable architecture.

## Overview

This project simulates a telemetry pipeline for IoT devices, processing the data using **Rust** for high-performance data ingestion and **Django** for backend services. Telemetry data is managed through **Redis**, tasks are processed asynchronously by **Celery**, and the system is monitored using **Prometheus** and **Grafana**.

- **IoT Devices** send telemetry data (e.g., temperature, humidity) to a backend.
- **Rust** ingests data and pushes it to **Redis**.
- **Django** manages the data, exposing APIs for devices and telemetry.
- **Celery** processes tasks asynchronously, such as aggregating data or triggering alerts.
- **Prometheus** scrapes system metrics, and **Grafana** provides dashboards to visualize the data.

---

## Technologies

- **Django**: For building the backend and exposing REST APIs.
- **Rust**: For high-performance data ingestion and processing.
- **Celery**: For asynchronous task processing.
- **Redis**: For message queuing and storing telemetry data.
- **Postgres**: For storing device and telemetry data in a relational database.
- **Prometheus**: For monitoring system metrics.
- **Grafana**: For visualizing Prometheus data in dashboards.
- **Docker**: For containerizing services (Redis, Postgres, Celery, Prometheus, Grafana).

---

## System Architecture

The system follows a microservice architecture where different services communicate with each other, primarily using **Redis** for queuing and communication.

1. **Django**: Handles HTTP requests, connects to Postgres for storing device and telemetry data, and exposes APIs.
2. **Rust**: Ingests telemetry data from IoT devices and pushes it into Redis for processing.
3. **Celery Worker**: Consumes telemetry data from Redis, processes it (e.g., aggregations, alerts), and stores the results back into Postgres.
4. **Prometheus/Grafana**: Used for monitoring system metrics, including Redis, Django, and Celery performance.

---

## Setup Instructions

### 1. Install Redis and Postgres (Locally)
If you are not using Docker for Redis and Postgres, you can install them locally:

- **Redis**: Follow the installation guide for your OS: [Redis installation](https://redis.io/docs/getting-started/)
- **Postgres**: Install PostgreSQL and configure the database.

---

### 2. Django Setup

1. **Install Dependencies**:

```bash
cd backend_django/
pip install -r requirements.txt
```

2. **Apply Database Migrations**:

```bash
python manage.py migrate
```

3. **Run the Django Development Server**:

```bash
python manage.py runserver
```

Your Django app will be running at [http://localhost:8000](http://localhost:8000).

---

### 3. Celery Setup

1. **Install Celery Dependencies**:

```bash
pip install -r requirements.txt  # Make sure Celery and Redis are installed
```

2. **Start the Celery Worker**:

```bash
celery -A visiontrack worker --loglevel=info
```

This will start the Celery worker that listens to the Redis queue for tasks.

---

### 4. Rust Service Setup

1. **Install Rust**: Make sure you have **Rust** installed on your machine. If not, install it from [rust-lang.org](https://www.rust-lang.org/).

2. **Build and Run the Rust Service**:

```bash
cd ingestion_rust/
cargo run
```

This service will begin listening for telemetry data and pushing it into Redis for processing.

---

### 5. Prometheus and Grafana

1. **Prometheus Setup**: Prometheus is set up to scrape Redis and other metrics. The configuration is in `infra/prometheus.yml`.

2. **Grafana Setup**:
   - Access **Grafana** at [http://localhost:3000](http://localhost:3000) with the default username `admin` and password `admin`.
   - Configure **Prometheus** as a data source.
   - Import the predefined dashboards (or create your own) to visualize Redis, Django, and Celery metrics.

---

## Running the Project

1. **Start Docker Services**: Run the following to start Redis, Postgres, and other services:

```bash
cd infra/
docker-compose up -d
```

2. **Start Django**: Run the Django server:

```bash
cd backend_django/
python manage.py runserver
```

3. **Start Celery**:

```bash
celery -A visiontrack worker --loglevel=info
```

4. **Start Rust Service**: Run the Rust service to push telemetry data into Redis:

```bash
cd ingestion_rust/
cargo run
```

5. **Monitor**: Access **Grafana** at [http://localhost:3000](http://localhost:3000) and configure Prometheus as a data source to view metrics.

---

## API Endpoints

- **POST /api/devices/**: Add a new device.
- **GET /api/devices/**: List all devices.
- **POST /api/telemetry/**: Add telemetry data linked to a device.
- **GET /api/telemetry/**: List all telemetry data.

---

## Monitoring

- **Prometheus** scrapes data from Redis and Django and exposes it on [http://localhost:9090](http://localhost:9090).
- **Grafana** can visualize this data, with dashboards available for **Redis** and **Django** metrics.

---
