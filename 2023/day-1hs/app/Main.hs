module Main (main) where

import Data.Char (digitToInt, isDigit)
import Data.List (isPrefixOf, tails)
import Data.Maybe (catMaybes, listToMaybe)

main :: IO ()
main =
  do
    content <- readFile "day-1.txt"
    print $ part1 content
    print $ part2 content

part1 :: String -> Int
part1 input =
  sum numbers
  where
    filterDigits s = [digitToInt c | c <- s, isDigit c]
    numbers = [numberFromDigits $ filterDigits line | line <- lines input]

part2 :: String -> Int
part2 xs = sum [numberFromLine line | line <- lines xs]

numberFromLine :: String -> Int
numberFromLine xs = numberFromDigits $ digitsFromLine xs

numberFromDigits :: [Int] -> Int
numberFromDigits xs =
  let (a, b) = firstAndLast xs
   in a * 10 + b

digitsFromLine :: String -> [Int]
digitsFromLine xs = catMaybes [digitFromPrefix i | i <- tails xs]

digitFromPrefix :: [Char] -> Maybe Int
digitFromPrefix [] = Nothing
digitFromPrefix xs@(c : _)
  | isDigit c = Just $ digitToInt c
  | otherwise = listToMaybe [d | (p, d) <- prefixDigits, p `isPrefixOf` xs]

prefixDigits :: [(String, Int)]
prefixDigits = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)]

firstAndLast :: [Int] -> (Int, Int)
firstAndLast xs = (head xs, last xs)

testInput :: String
testInput = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"

testInput2 :: String
testInput2 = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"
