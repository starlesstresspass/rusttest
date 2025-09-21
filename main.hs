main = putStrLn "Hello, World!"
doubleMe x = x * 2
squareMe :: Num y => y -> y  
squareMe x = x * x
doubleSmallNumber :: Int -> Int 
doubleSmallNumber x = if x > 100
                        then x 
                        else x*2
removeNonUppercase :: [Char] -> [Char]
removeNonUppercase st =[c | c <- st, c `elem` ['A'..'Z']]
succTwo :: Num x => x -> x 
succTwo x = x + 1

--factorial :: Num x => x -> x
--factorial x = map  [1..x]
