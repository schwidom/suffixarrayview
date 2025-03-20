
The suffixarrayview creates a view of an internally created suffix array from one or more files.

Usage: suffixarrayview [OPTIONS] [FILES]...

Arguments:
  [FILES]...  

Options:
  -r, --right                    align the output ro the right
  -l, --left                     align the output to the left -l and -r is treated as -l
  -f, --filename                 shows the filenames
  -i, --index                    shows the filename indices
  -w, --width <WIDTH>            maximum dataset length in chars / bytes, default is unlimited
  -b, --binary                   binary mode, default is UTF8 mode
  -t, --tokenregex <TOKENREGEX>  default is every char / byte, defines the start of all suffixes
  -v, --verbose                  
  -h, --help                     Print help
  -V, --version                  Print version

Examples:

$ suffixarrayview -w 60 -t '\b[[:alnum:]]' <( echo "   
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")

Duis aute irure dolor in reprehenderit in voluptate velit es
Excepteur sint occaecat cupidatat non proident, sunt in culp
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
Ut enim ad minim veniam, quis nostrud exercitation ullamco l
ad minim veniam, quis nostrud exercitation ullamco laboris n
adipiscing elit, sed do eiusmod tempor incididunt ut labore 
aliqua. Ut enim ad minim veniam, quis nostrud exercitation u
aliquip ex ea commodo consequat. Duis aute irure dolor in re
amet, consectetur adipiscing elit, sed do eiusmod tempor inc
anim id est laborum. 
...
deserunt mollit anim id est laborum. 
do eiusmod tempor incididunt ut labore et dolore magna aliqu
dolor in reprehenderit in voluptate velit esse cillum dolore
dolor sit amet, consectetur adipiscing elit, sed do eiusmod 
dolore eu fugiat nulla pariatur. Excepteur sint occaecat cup
dolore magna aliqua. Ut enim ad minim veniam, quis nostrud e
ea commodo consequat. Duis aute irure dolor in reprehenderit
...
velit esse cillum dolore eu fugiat nulla pariatur. Excepteur
veniam, quis nostrud exercitation ullamco laboris nisi ut al
voluptate velit esse cillum dolore eu fugiat nulla pariatur.

$ suffixarrayview -w 60 <( echo "
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")
...
abore et dolore magna aliqua. Ut enim ad minim veniam, quis 
aboris nisi ut aliquip ex ea commodo consequat. Duis aute ir
aborum. 
ad minim veniam, quis nostrud exercitation ullamco laboris n
adipiscing elit, sed do eiusmod tempor incididunt ut labore 
aecat cupidatat non proident, sunt in culpa qui officia dese
agna aliqua. Ut enim ad minim veniam, quis nostrud exercitat
aliqua. Ut enim ad minim veniam, quis nostrud exercitation u
aliquip ex ea commodo consequat. Duis aute irure dolor in re
am, quis nostrud exercitation ullamco laboris nisi ut aliqui
amco laboris nisi ut aliquip ex ea commodo consequat. Duis a
amet, consectetur adipiscing elit, sed do eiusmod tempor inc
anim id est laborum. 
...


$ suffixarrayview -w 10 -b <( echo "
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")
[10]
[10, 76, 111, 114, 101, 109, 32, 105, 112, 115]
[32, 68, 117, 105, 115, 32, 97, 117, 116, 101]
[32, 69, 120, 99, 101, 112, 116, 101, 117, 114]
[32, 85, 116, 32, 101, 110, 105, 109, 32, 97]
...
[97, 116, 117, 114, 46, 32, 69, 120, 99, 101]
[97, 117, 116, 101, 32, 105, 114, 117, 114, 101]
[98, 111, 114, 101, 32, 101, 116, 32, 100, 111]
[98, 111, 114, 105, 115, 32, 110, 105, 115, 105]
[98, 111, 114, 117, 109, 46, 10]
[99, 97, 101, 99, 97, 116, 32, 99, 117, 112]
...
[120, 32, 101, 97, 32, 99, 111, 109, 109, 111]
[120, 99, 101, 112, 116, 101, 117, 114, 32, 115]
[120, 101, 114, 99, 105, 116, 97, 116, 105, 111]


