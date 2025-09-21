--main = putStrLn "Hello, World!"
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
--sayHello name@(x:xs) = "Hello " ++ name ++ ", your name starts with " ++ [x]
--sayHello "Alice"
--densityTell :: (RealFloat a) => a -> a -> String
--densityTell density 
  --   |
    --  density < 1.2 = "Your gonna fly!"
      -- |
     -- density <= 1000.0 = "Have fun swimming"
      -- |
     -- otherwise = "your gonna sink lmaoo"
     
initials :: String -> String -> String
initials firstname lastname = [f] ++ "." ++ [l] ++ "."
         where (f:_) = firstname
	       (l:_) = lastname
data Operation = Add | Subtract | Multiply | Divide deriving (Show, Read)
getInput :: IO(Double, Double, Operation) 

getInput = do
    putStrLn "Enter the first number: "
    num1 <- getLine
    putStrLn "Enter the second number: "
    num2 <- getLine
    putStrLn "Enter an operator: "
    op <- getLine
    return(read num1, read num2, read op)
calculate :: Double -> Double -> Operation -> Double
calculate x y Add = x + y
calculate x y Subtract = x - y
calculate x y Multiply = x * y
calculate x y Divide = x / y

main :: IO()
main = do
   (num1, num2, op) <- getInput
   let result = calculate num1 num2 op
   putStrLn $"The result is: " ++ show result
