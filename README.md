# Neocat
Made to combine cat and lolcat, maybe some unique features 


Example command:  
`neocat -r /etc/passwd`, prints contents of /etc/passwd in rainbow  

Usage: `neocat {ARG/FILE/STRING}`  
  args:
  -c -- color, basic colors, red, blue, green, etc | -r -- rainbow, makes output text rainbow
  
  file: outputs full file contents, can be colored with `neocat -c COLOR FILE` or `neocat -r FILE`
  
  string: if a string that isnt a file is used it just prints the string, can be colored `neocat -c COLOR STRING` or `neocat -r STRING`
  
Similar to lolcat, you can pipe a command into it like this `pacman -Q | wc -l | neocat -r`, that would get the package count and make it rainbow on arch linux  

TODO: windows support?
