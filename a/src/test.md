Generate a rust function that generates N subset of the following array A: [9, 8, 9, 5, 3, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1].
Each subset must be made out of unique element of the array A, and add up to the sum target X.
You can assume that N divides X without reminder, so each subset must always sum up to the target X.
The function should return a vector containing the N subset.
The sum of all four subset's sum should be equal to N*X.
Consider using some backtracking features and keep track of used values in order to achieve the exact result.
This function should work on any given array that can live up to the said assumptions.

Here is an example
N = 4
X = 12
A = [9, 8, 9, 5, 3, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1]

subsets 1: [9, 2, 1]
subsets 2: [9, 1, 1, 1]
subsets 3: [8, 2, 2]
subsets 3: [3, 5, 1, 1, 1, 1]