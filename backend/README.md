# Backend documentation

The backend is written in Rust. Running the project launches a REST API that can be used to generate allocation data.

## Deployment

The backend can be run in a Docker container. It can be be deployed to any cloud architecture that supports containers.

```
$ cd backend
$ docker build -t allocation-backend
$ docker run -v DIR_WITH_INPUT_ZIP:/app/raw_input -p 8080:8080 allocation-backend
```

Make sure that DIR_WITH_INPUT_ZIP is a folder on your machine that contains .zip files with the allocation specifications. These .zip files are processed on the container start.

TODO: push Docker image to a registry.

## Running the project locally for testing purposes

You can launch the API locally by first installing Rust and then running `cargo run`. The API will be available at http://127.0.0.1:8080 .

## Endpoints

The endpoints are documented with OpenAPI documentation. A Swagger UI is generated on top of the documentation at address /swagger-ui/ (remember the last /) when running the APIs somewhere.

The Swagger UI can be used also to test the endpoints.

An example deployment, with Swagger UI, can be found at TODO.

## Concepts

The project utilizies the following concepts:

- _round_: Allocations are organized in various rounds. One round can contain any number of allocations for addresses. Typically rounds start from 1 but as long as the number is increasing you can start from any (small) number.
- _root_: Refers to the root of a Merkle tree. This root dictates which addresses are eligible for how many tokens. One root exists for one round.

## Adding new data for allocations

Once you launch the backend the project first extracts all of the allocation information from files. The information is then stored in the program memory, for the backend/API endpoints to utilize.

If you add new allocation files you need to restart the backend so it starts processing the files.

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

Addition of new data has to be connected with reinitializing the backend docker container.

New data (new file with data) corresponds only to the given round. The aggregation happens on start of the backend.

## Notes on performance

If there are a lot of entries in the input files it may take a while to get the backend started. Processing a file with a million entries may take an hour. The program is single-threaded. The backend will output "API ready" once everything has been processed.

The main problem with this is that the same processing is performed every time the backend is started, because all the trees are only stored in memory. This approach may need to be revised in the future.

The main bottlenect in the performance is calculating the hash values for the tree. There isn't much that can be done to improve that directly.

## Other notes

### Extraction of the capital from the smart contract by the owner account

At this point, there is no need to have "extraction of the capital by the owner/foundation" functionality, but there is a way to do this. This means that the owner of the smart contract has to be properly secured/safe.

Imagine there is 100 tokens on the smart contract. Malicious owner account can submit root that corresponds to a tree that would send the 100 to itself, even though this account is not eligible at all. 

### Removal of root(s) from smart contract is not possible

It is not possible to remove or overwrite root(s) in the smart contract. It is required for the protocols to store root in the smart contract that has been checked that is correct.

In can case that a mistake happens here is what can be done:

- If an account has been omitted or account has lower allocation that it should have a new root that corrects the state can be added.
- If an account has been added or has higher allocation 
1. In case account owner stores new root that does corrects this error, someone with the knowledge of the tree that has the mistake can potentially provide the information to the given account that can claim more than it should.
1. Possible way would be to extract all capital through storing a new merkle root that would allow for sending the remaining capital on the smart contract to a newly deployed distribution smart contract (it would likely have to go through a third account). This newly redeployed contract would have to have trees adjusted for the already claimed tokens in the previous one.