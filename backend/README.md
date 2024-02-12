# Backend documentation

The backend is written in Rust. Running the project launches a REST API that can be used to generate allocation data.

## Deployment

The backend is packaged in a Docker container. It can be be deployed to any cloud architecture that supports containers.

TODO

## Running the project locally for testing purposes

You can launch the API locally by first installing Rust and then running `cargo run`. The API will be available at http://127.0.0.1:8080 .

## Endpoints

The endpoints are documented with OpenAPI documentation. A Swagger UI is generated on top of the documentation at address /swagger-ui/ when running the APIs somewhere.

The Swagger UI can be used also to test the endpoints.

An example deployment, with Swagger UI, can be found at TODO.

## Concepts

The project utilizies the following concepts:

- _round_: Allocations are organized in various rounds. One round can contain any number of allocations for addresses. Typically rounds start from 1 but as long as the number is increasing you can start from any (small) number.
- _root_: Refers to the root of a Merkle tree. This root dictates which addresses are eligible for how many tokens.

## Adding new data for allocations

Once you launch the API the project first extracts all of the allocation information from files. The information is then stored in the program memory, for the API endpoints to utilize.

The input files should be located in the _./raw_input_ folder.

The files have the following characteristics:

- The files should be JSON files compressed with ZIP. Don't use encryption or other non-default options
- Each ZIP file should have the format: raw_X.zip where X is the round number
- Each ZIP file should contain only one file with the same name, but with file extension _.JSON_
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

The addresses in the JSON files should be Starknet wallet addresses for the recipients of the allocation. The amounts should be the amount in its base units: 1 full STRK token is expressed as _1000000000000000000_. No decimal amounts are allowed.
