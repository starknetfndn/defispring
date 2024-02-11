# Backend documentation

The backend is written in Rust. Running the project launches a REST API that can be used to generate airdrop data.

## Deployment

The backend is packaged in a Docker container. It can be be deployed to any cloud architecture that supports containers.

TODO

## Running the project locally for testing purposes

You can launch the API locally by first installing Rust and then running `cargo run`. The API will be available at http://127.0.0.1:8080 .

## Endpoints

The endpoints are documented with OpenAPI documentation. A Swagger UI is generated on top of the documentation at address /swagger-ui/ when running the APIs somewhere.

The Swagger UI can be used also to test the endpoints.

An example deployment, with Swagger UI, can be found at TODO.

## Project structure

The project has the following notable folders:

- _contract_: Contains the Cairo contract and everything related to it
- _src_: Contains the Rust backend
- _tests_: Contains unit tests for the backend

## Concepts

The project utilizies the following concepts:

- _round_: Airdrops are organized in various rounds. One round can contain any number of airdrops for addresses. The first round is number 1 and each subsequent round should increase the number by one

## Adding new data for airdrops

Once you launch the API, the project first extracts all of the airdrop information from files. These files are located in the _src/raw_input_ folder.

The files have the following characteristics:

- The files should be JSON files compressed with ZIP. Don't use encryption or other non-default options
- Each ZIP file should have the format: raw_X.zip where X is the round number
- Each ZIP file should contain only one file with the same name, but with file extension .JSON
- Each JSON file should have the following format:

```
[
  {
    "address": "0x11",
    "amount": "123"
  },
  {
    "address": "0x12",
    "amount": "234"
  },
]

```

The addresses in the JSON files should be Starknet wallet addresses for the recipients of the airdrop. The amounts should be the amount in its basic units: TODO.
