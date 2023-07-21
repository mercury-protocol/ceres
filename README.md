# Mercury Verifier SDK

This SDK can be used by developers to create new data collectors. Data collectors allow data to be collected, and then sold on the Mercury Protocol. Data collectors can be created for any imaginable data type.    

They have two parts:   
- **collector**:   
The collector can be written in any language and in any structure. It's goal is to get data from a source in a pre-standardized format and transmit it to a data union admin node.   
    
- **verifier**:   
The verifier check the format, structure, and layout of the data, verifies that everything is in the correct format and structure, then finally generates a zero-knowledge proof of this. This proof then later can be uploaded on-chain, and vefified by anyone.  
   
Verifiers must be written in the Rust programming language, using our SDK.    

## TODO: 
- this SDK has to be a command line tool that can create a new collector-verifier project structure.   
- ```new``` command will generate a folder with the given argument name -> inside the folder there will be a subfolder named ```collector``` and a subfolder named ```verifier```.   The collector will only contain a README.md, and it will be completely up to the creator to fill it with code that collects some data.   
The verifier will create a new RiscZero template. <- it can literally call the ```cargo risczero new``` command with the folder name set to verifier. We can ship the risczero binary along with the SDK. 
- generate a PR.md file, which will contain questions that the new collector pull request must answer -> collector creators must fill this out before they can create a new PR  
    
### THE HOST:
- in the host code, what the programmer must do is open some data file, do any preprocessing, and convert it to a binary array, which then will automatically be passed into the guest program   
**- there must be a ```generate``` command, that will autogenerate all the necessary code that will always be the same + fill in the parts that the user added to the files**   
**- there must be a standard name for the METHOD_NAME_ELF and METHOD_NAME_ID fields as well -> these will also be automatically created by the SDK**
- the pre-made code for the host program must contain the following: take the byte array with the name ```data```, create a new ```ExecutorEnv``` and pass that in as an input    
- next it will run the guest code and prove the receipt   
- then it must implement code for serializing the receipt / storing it locally / upload it to Filecoin / ( if we can later on ) creating a SNARK from the STARK and uploading and verifying it on-chain
- it must also store the given CID that was returned from the journal    
- the argument to the program must be the path to the data file that we are verifying   
    
### THE GUEST:
- the code that is pre-implemented by us is that we take the byte array that was passed into the guest, and create the CID for the given data   
- we commit the created CID to the env, then return    
- the rest of the logic is completely up to the creator of the collector, and can and must do any verification on the data that's necessary   

Our SDK will have a ```compile``` command, that will generate the complete collector-verifier that can then be uploaded to Mercury.    
It will pase together the host-guest from the user created parts with the precompiled parts. It can have a command that verifies that everything complies and the ZK proof verifies successfully.
Uploading will look like this:  
1. generate code   
2. submit PR to GitHub   
3. approve PR   
4. We add the given data type + verifier image ID to our smart contract   
5. We add the verifier code repo to our monorepo for all verifiers   
6. The data type can now be collected and sold through Mercury    