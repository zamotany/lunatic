package main

import (
	"fmt"
	"zamotany/lunatic/scanner"
)

var sourceInput string = `                    --1
--[[                                          --2
	this is multiline comment 1 then ()         --3
]]                                            --4
-- defines a factorial function               --5
function fact (n)                             --6
	if n == 0 then                              --7
		return 1                                  --8
	else                                        --9
		return n * fact(n-1)                      --10
	end                                         --11
end                                           --12
                                              --13
local a = 'some string\''                     --14
local b = "some string\""                     --15
local c = [[                                  --16
	multiline                                   --17
	string                                      --18
]]                                            --19
                                              --20
print("enter a number:")                      --21
a = io.read("*number")        -- read a number  22
print(fact(a))                                --23
`

func main() {
	scanner := scanner.NewScanner(&sourceInput)
	if _, err := scanner.ScanTokens(); err == nil {
		fmt.Println(scanner.DebugString())
	} else {
		fmt.Println(err)
	}
}
