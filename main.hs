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
luckySeven :: (Integral a) => a -> String
luckySeven 7 = "Your lucky!"
luckySeven x = "Unlucky bastard!"
headTwo :: [a] -> a 
headTwo [] = error "Cant call head on an empty array/tuple."
headTwo (x:_) = x
lengthTwo :: (Num b) => [a] -> b 
lengthTwo [] = 0
lengthTwo (_:xs) = 1 + lengthTwo xs 
sumTwo :: (Num a) => [a] -> a 
sumTwo [] = 0
sumTwo (x:xs) = x + sumTwo xs  
