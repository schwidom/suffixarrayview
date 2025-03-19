
The suffixarrayview creates a view of an internally created suffix array from one or more files.

Usage: suffixarrayview [OPTIONS] [FILES]...

Arguments:
  [FILES]...  

Options:
  -w, --width <WIDTH>            maximum dataset length in chars / bytes, default is unlimited
  -b, --binary                   binary mode, default is UTF8 mode
  -r, --regex <REGEX>            input regex, without this regex every suffix goes to the end of the file
  -t, --tokenregex <TOKENREGEX>  default is every char / byte, defines 
  -v, --verbose                  
  -h, --help                     Print help

