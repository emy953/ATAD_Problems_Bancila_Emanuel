# ATAD_Problems_Bancila_Emanuel

This repository contains 10 easy problems and 4 medium ones from https://www.hackerrank.com/.
# Explanations:
# Solve me first
As the comment says, we needed to type println!("{}", _num_1 + _num_2); in order to rpint the sum of the numbers.
# Simple Array Sum
The function had to be completed with ar.iter().sum() in order to return the sum of all elements of the array ar as i32.
# Compare the Triplets
The function uses varialbes alice_score and bob_score to calculate the score for both of them. The a and b vectors are iterated with a_val and b_val iterators using iter().zip(). The scores are returned as a vector of i32.
# A very Big Sum
The function uses ar.iter().sum() in order to iterate over the vector of i64 and calculate the sum. It is returned as a value of type i64.
# Diagonal Difference
The function calucaltes the sums of the diagonals of the square matrix using the formula of arr[i][i] for the first diagonal and arr[i][n - i - 1] for the second diagonal. The function returns the absolute value of the difference of both sums.
# Plus Minus
The function iterates over the array arr and counts all the positive, negative and zero elements. It calculates the ratios of each of them by dividing the count to the total number and prints the ratios.
# Staircase
The function starts with n-1 spaces and 1 hash and prints the staircase incrementing the number of hashes and decrementing the number of spaces.
# Min-Max Sum
The function gets the biggest and the smallest elements of the array arr.iter().min().unwrap() / arr.iter().max().unwrap(). The minimum sum is the total sum minus the biggest elemet and the maximum sum is the total sum minus the smallest element.
# Birthday Cake Candles
The function gets the maximum height of the array and counts how many elements have the value equal to this maximum, returning this count as i32.
# Time Conversion
The function splits the String in 3 Strings: perion, hour and minute_and_second. It uses the period in order to manipulate the hour treating the special cases of 12. In the end the function returns a String formatted accordingly.
# Medium Problems
# Extra Long Factorials
The function uses a vector of digits starting with 1. It does the multiplications with each number from 2 to n using the digits, adding the digits of the carry resulted from the multiplication to the array after each step.
# Organizing Containers of Balls
After analyzing the problem, we can see that the requested swaps are Possible only if, for each type of ball, there exists a container with the capacity equal to the total number of balls of that type. So, if there are 4 red balls and there is no container with the size equal to 4, the solving is Impossible.
The function calculates the sum on rows as the total number of balls of each type and the sum on columns as the capacity of each container. It checks for each type of ball that the condition is true, if it is not it returns Impossible.
# Encryption
The function calculates the sqrt value of the length and extracts the floor and ceiling values using the floor() and ceil() functions. Then, it splits the string in rows of length equal to the ceiling value. After this, it creates the result by adding the elements of each column in order, separated by spaces. In the end it removes the last space added and returns the result String.
# The Time In Words
The function uses a string of words in order, from 0 to 30 (num_to_word[15] = "fifteen"). It then returns a formatted string according to the special cases for minutes. 
