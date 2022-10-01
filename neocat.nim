import os
import terminal
import strutils
import typetraits

let validColors = @[fgRed, fgGreen, fgYellow, fgBlue, fgMagenta, fgCyan]

proc makeRainbow(msg: string, colors: seq[ForegroundColor]):string = 
  var ind = 0
  var output:seq[string] = @[]
  for letter in msg:  
    output.add(ansiForegroundColorCode(colors[ind]) & letter & ansiResetCode)
    ind = (ind + 1 + colors.len) mod colors.len
  return output.join("")

if paramCount() >= 1:
  case paramStr(1)
  of "-c":
      var color = "fg" & capitalizeAscii(paramStr(2))
      if color != "fgRainbow":
        try:
          echo ansiForegroundColorCode(parseEnum[ForegroundColor](color)) & readFile(paramStr(3)) & ansiResetCode
        except ValueError:
          echo "invalid color: ", paramStr(2)
        except IOError:
          echo ansiForegroundColorCode(parseEnum[ForegroundColor](color)) & paramStr(3) & ansiResetCode
      else:
        try:
          echo makeRainbow(readFile(paramStr(3)), validColors)
        except IOError:
          echo makeRainbow(paramStr(3), validColors)
  of "-r":
    if isatty(stdin) == false:
      echo makeRainbow(stdin.readLine, validColors)
    else:
      try:
        echo makeRainbow(readFile(paramStr(2)), validColors)
      except IOError:
        echo makeRainbow(paramStr(2), validColors)
  else:
    try:
      echo readFile(paramStr(1))
    except IOError:
      echo paramStr(1)

elif paramCount() == 0:
  if isatty(stdin) == false:
    echo stdin.readLine
