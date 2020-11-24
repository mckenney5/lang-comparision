-- |This program takes in user input and says hello
-- Compile with: ghc --make haskell.hs

import System.IO
main = do  
    putStr "Enter your name: "
    hFlush stdout
    name <- getLine  
    putStrLn ("Hello " ++ name ++ "!\n")

