# Ceres

## What is Ceres?

> *In Roman mythology, Ceres was the god of agriculture and harvest. The overseer of cultivating and utilizing resources.*

Ceres is a simple command line tool that helps developers create **data collectors** and **data verifiers**. These two things enable *any* data to be collected and its structure and integrity to be *trustlessly* verifed. This data can then be rented out on the **Mercury Data Marketplace**, and computations (AI/ML/data science etc) can be run on it on the **Mercury Compute Network**.   
   

### Data collectors    
Data collectors are responsible for collecting data from some source in a pre-defined, standardized format and transmitting it to a data DAO node. Data Collectors can be created in any programming language. They have one purpose: **to harvest data and transport it**. Ceres creates code for you that you can use to get the address of a data DAO node in our peer-to-peer protocol, and transfer the data to it. Currently we provide this in Go, JavaScript, and Python, but more language implementations (Rust, C++, Swift) are planned.
    
### Data verifiers    
So if anyone can create and run data collectors, what prevents dishonest actors from abusing the system? Data verifiers. 