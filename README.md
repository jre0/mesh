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

If the mesh is made so some triangles touch each other but do not actually share a vertex, I could keep the same data structure but would need to change 
some of the functions to check for connectivity by distance threshold instead of by face vertex index.

Here are some thoughts: For problem 2, I was on the right track. I was trying to write a condition that checks if two triangles share the same edge. If they do and their edges run in opposite ways, they are healthy. If their shared edges run the same direction, the triangles are faces two different ways which is bad.I did not get to problem 4 unfortunately. 
For problem 5, I did not finish it. But the idea was to show 

