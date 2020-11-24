\ forth (GNU Forth aka gForth)
\ takes in the users name and says hello
\ to run: gforth forth.fs

1024 CONSTANT SIZE \ size of max input

: TEXT ( -- )  PAD SIZE BL FILL WORD COUNT PAD SWAP MOVE ; \ sets up a string

: GREET ( -- ) CR ." Enter your name: " \ 
       TIB SIZE ACCEPT #TIB !  0 >IN !
       1 TEXT CR ." Hello "
       PAD 1024 -TRAILING TYPE 33 EMIT 10 EMIT ;

GREET
BYE

