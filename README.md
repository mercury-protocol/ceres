
![banner-ceres](https://i.imgur.com/yQmEYC5.png)

# Ceres

## What is Ceres?

> *In Roman mythology, Ceres was the god of agriculture and harvest. The overseer of cultivating and utilizing resources.*

Ceres is a simple command line tool that helps developers create **data collectors** and **data verifiers**. These two things enable *any* data to be collected and its structure and integrity to be *trustlessly* verifed. This data can then be rented out on the **Mercury Data Marketplace**, and computations (AI/ML/data science etc) can be run on it on the **Mercury Compute Network**.   
   

### Data collectors    
Data collectors are responsible for collecting data from some source in a pre-defined, standardized format and transmitting it to a data DAO node. Data Collectors can be created in any programming language. They have one purpose: **to harvest data and transport it**.     
Ceres creates code for you that you can use to get the address of a data DAO node in our peer-to-peer protocol, and transfer the data to it. Currently we provide this in Go, JavaScript, and Python, but more language implementations (Rust, C++, Swift, Java) are planned.
    
### Data verifiers    
So if anyone can create and run data collectors, what prevents dishonest actors from abusing the system? Data verifiers.    
Data verifiers take data from a data collector and analyze its content and structure to verify the authenticity and integrity of the data. At the end of this process, they generate a zero-knowledge proof, and log the content identifier of the data. This proof, along with the content identifier can be sent to any third party requiring assurance that the data is correct.   
      
Ceres uses [RiscZero](https://github.com/risc0/risc0), a general computing platform that can generate a zero-knowledge proofs of any computation executed in the *zkVM*.   
Zero-knowledge proofs allow one party (i.e. *the data collector*) to convince another party (i.e. *the data buyer*) that something is true (i.e. *that the data is authentic and correct*) without revealing all the details (i.e. *private data*).   
     
## How to build collectors & verifiers with Ceres    
Ceres helps developers with creating data collectors and verifiers, so that they only have to focus on the logic of the data collection and verification, while Ceres takes care of transporting the data, generating the RiscZero project, and preparing a pull request to our [data collectors-verifiers repo](https://github.com/mercury-protocol/mcy-data-collectors).
The architecture of the **data collector** program is completely up to the developer. Data can come from any source, and we don't pretend to have an answer to what data types are worth collecting.   
The developer is responsible for implementing the data collection logic, but can use one of the data transport implementations provided by Ceres to transfer the data to a data DAO node.   

The **data verifier** program is a RiscZero binary. Ceres generates you two files: `hostlib.rs` and `guestlib.rs`. They contain one method each, and it is the developer's responsibility to implement these methods.    
In `hostlib.rs` we have a method called `prepare`, that takes a path to a data file, and runs any preparatory work on it. It must return a `Vec<u8>` of the data.    
In `guestlib.rs` we have a method called `verify`, which takes the `Vec<u8>` of the data, and returns a `bool`, indicating whether the verification was successful or not. Here, you must implement any authentication/verification/structural integrity etc. checks on the data that you deem necessary.    
    
Once your program is ready, you must submit a pull request to our [data collectors-verifiers repo](https://github.com/mercury-protocol/mcy-data-collectors) with a link to your code and also an explanation with what the code does. Ceres has tools that help you with this process.    
     
## Usage
| Command | Description | Required Argument | Optional Flags | 
|---------|-------------|-------------------|----------------| 
| init    | Creates a new collector-verifier project | NAME - the name of the project to be created | --go: generate code for the data transport in Golang<br>--js: generate code for the data transport in JavaScript<br>--py: generate code for the data transport in Python |
| gen     | Take the code in `/verifier` and create a complete RiscZero program ready to verify data | N/A | N/A |
| build   | Take the code in `/verifier` and create an executable binary | N/A | N/A |
| new-pr  | Walks you through the process of creating a new pull request for this data collector-verifier. Outputs a `pr.md` file that can be added to the pull request | N/A | N/A |
| add-pr  | Adds your collector-verifier to the [data collectors repo](https://github.com/mercury-protocol/mcy-data-collectors) | PR_FILE - path to the pr.md file | N/A |


