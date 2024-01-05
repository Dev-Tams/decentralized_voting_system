# Decentralized Voting System

Welcome to the Decentralized Voting System, an innovative project set to transform the voting landscape. This initiative harnesses the capabilities of blockchain technology to establish a secure and tamper-proof environment for casting votes.

This Rust Smart contract implements a voting system on the Internet Computer, allowing users to securely participate in various elections. Leveraging the Internet Computer's smart contract capabilities, it effectively manages user accounts, records, and transactions with transparency and reliability. The primary objective is to provide users with a secure, transparent, and efficient platform for participating in diverse voting scenarios, from community-driven decisions to formal elections.


## Table of Contents

- [Features](#features)
- [Technologies](#technologies)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [API Endpoints](#api-endpoints)
- [Contributing](#contributing)
- [License](#license)

## Features



#### Secure Voting
Users can securely cast their votes with the implementation of blockchain technology, guaranteeing the integrity of the voting process.

#### Structural Improvements
The project introduces new traits (`Storable` and `BoundedStorable`) and structures (Election, Voter, Ballot, and Vote) to enhance the overall management of elections, voters, and ballots.

#### Query and Functionality Expansion
New queries and functions empower users to retrieve detailed election information, obtain results, check the status of ongoing and available elections, create new elections, manage voter registration and ballot creation.

#### Error Handling Enhancement
The error-handling mechanism has been fortified with additional variants to address specific scenarios, ensuring a more robust and reliable system.

#### Advanced Result Calculation
The system includes a sophisticated `calculate_election_results` function, providing a HashMap of candidate names and their respective vote counts. This feature contributes to comprehensive result analysis.

#### Internal Helper Functions
Internal helper functions have been introduced for efficient management of user and vote data, optimizing system performance.


## Technologies
- Internet Computer Protocol (ICP)
- Rust for backend logic
- DFX - Internet Computer CLI 
- Cargo
- Candid
- Motoko for Internet Computer compatibility
- HTML, CSS, and JavaScript for the frontend



## Getting Started

### Environment Setup:

#### Rust Programming Language:

- Install Rust on your machine. You can use the official Rust installation instructions: `www.rust-lang.org` or run the following command toInstall Rust.
```curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh```



#### Installing Candid Extractor
- Run the following command to install Candid Extractor:

```cargo install candid-extractor```

#### Install DFX
```DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"```




### Running the projext locally


To run the project locally, follow these steps:

1. Clone the repository: `git clone https://github.com/Dev-Tams/decentralized-voting-system.git`
2. create directory `cd decentralized-voting-system`

## Usage

1. Start the backend server: [Provide instructions]

##### Start the replica (backend server) in the background
```dfx start --background```

#### Deploy canisters and generate Candid interface
```dfx deploy```

3. Generate Candid Interface (if Backend Changes):
- If you make changes to your backend canister, generate a new Candid interface using:

```npm run generate```

2. Open the frontend in a web browser: 

- Start the frontend development server
```npm start``

This will start a development server at http://localhost:8080,
proxying API requests to the backend at port 4943.

3. Interact with the system using the provided forms and chat interface.

- Open your web browser and go to http://localhost:8080 to access the frontend.
- Use the provided forms and chat interface to interact with the decentralized voting system.

## API Endpoints

- `/api/add_user` - Add a new user.
- `/api/add_ballot` - Add a new ballot for an election.
- `/api/cast_vote` - Cast a vote for a candidate in an election.
- `/api/calculate_results/:electionId` - Calculate and retrieve the results for a specific election.


## Contributing

We welcome contributions from the community! If you'd like to contribute, please follow these guidelines:

1. Fork the repository.
2. Create a new branch for your feature: `git checkout -b feature-name`
3. Make your changes and commit: `git commit -m 'Add new feature'`
4. Push to the branch: `git push origin feature-name`
5. Create a pull request.

## License

This project is licensed under the [MIT License](LICENSE).