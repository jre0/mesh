# Mesh Test

## Install 
If you don't have rust, please follow the instructions here: 
https://www.rust-lang.org/learn/get-started

## Test
Changed ```src/config.rs``` to have absolute path to ```src/test_data``` and ```src/test_output```
cd into root folder and run ```cargo test``` 
Cargo test will build and run the tests if you are in the root of the project (one up from src).

## Discussion 
I choose to use a flat array (Vec) of vertex vector components and flat array of faces (vertex indices) because it usually makes it simpilar to transform data
for reading, writing, and switching to different structures like Vector3. If the mesh is made so some triangles touch each other but do not actually share a vertex, I could keep the same data structure but would need to change some of the functions to check for connectivity by distance threshold instead of by face vertex index only.

In testing, I need to do some assert_eq! macros to test validity of the functions. I would have done this by some manual edity and inspection in Blender perhaps.

Unfortunately, I did not start problem 3.

Problem 4 is under the assumption that we want to select any face that is touching another face with a similar normal (same or less than given angle).

I did not finish problem 5 but here is the strategy. When I find a short edge from face vertex indices, I need to delete one vertex.Before that, I must selected adjacent faces and change their to-delete vertex to be the surviving vertex of the short edge. Finally, I need to delete the vertex and need to delete the faces that collapses into edges (but some faces survive).



