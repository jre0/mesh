# Mesh Test

## Install 
If you don't have rust, please follow the instructions here: 
https://www.rust-lang.org/learn/get-started

## Test
Changed ```src/config.rs``` to have absolute path to ```src/test_data``` and ```src/test_output```
cd into root folder and run ```cargo test``` 
Cargo test will build and run the tests if you are in the root of the project (one up from src).

## Discussion 
I choose to use a flat array (Vec) of vertex components and flat array of faces (vertex indices) because it usually makes it simpilar to transform data
for reading, writing, and switching to different structures like Vector3.  

