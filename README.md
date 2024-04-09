# Rust S3 Client with MinIO Example
This repository showcases a Rust application that interacts with an S3-compatible MinIO service for basic object storage operations. It demonstrates creating a bucket, uploading an object, listing objects within a bucket, retrieving an object, and deleting an object.
It relies on [rust-s3 v0.33.0](https://crates.io/crates/rust-s3) cargo crate for bucket operations. It is a simple demo app to explore rust-s3 crate crate usage.

# Prerequisites
Before running this project, ensure you have Docker and Docker Compose installed on your machine. This is required to run the MinIO server locally. Additionally, you should have Rust and Cargo installed to compile and run the Rust application.

# MinIO Setup with Docker Compose
Included in this repository is a docker-compose.yml file which defines a MinIO service. To start the MinIO server:

1. Navigate to the project directory.
2. Run docker-compose up -d to start the MinIO server in detached mode.
The MinIO server is accessible at http://localhost:9000 (for the S3 API) and http://localhost:9001 (for the MinIO Console).

Credentials for accessing MinIO are predefined in the docker-compose.yml file:

- User: minio_user
- Password: minio_password

You can change these values in the docker-compose.yml file as needed.

# Application Configuration
Before running the application, set up the environment variables required for connecting to the MinIO server. Copy the .env.example file to .env and adjust the values to match those used in the docker-compose.yml file:

```
MINIO_ACCESS_KEY=minio_user
MINIO_SECRET_KEY=minio_password
```

# Running the Rust Application
With the MinIO server running and the environment variables set, you can now run the Rust application using Cargo:
```rust
cargo run
```

The application performs the following operations in sequence:

1. Attempts to create a bucket named rusty-bucket (if not already present).
2. Uploads a text file named test_file with content "Rusty Bucket Testing" to the bucket.
2. Lists the contents of the bucket.
4. Retrieves and prints the contents of test_file.
5. Deletes test_file from the bucket.

# Accessing MinIO Console
To view the MinIO Console and manage buckets and objects through a UI, navigate to http://localhost:9001 in your web browser. Log in with the credentials provided in the docker-compose.yml file.



