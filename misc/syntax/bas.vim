if exists("b:current_syntax")
	finish
endif

syntax keyword basKeyword
	\ PRINT
	\ INPUT
	\ LET
	\ IF
	\ THEN
	\ GOTO
	\ END
	\ SCREEN
	\ CLEAR
	\ COLOR
	\ LINE
	\ TO
	\ DOT
	\ CIRCLE

syntax match  basNumber "\v<\d+>"
syntax region basString start=/"/ end=/"/
syntax match  basComment "\vREM.*$"

syntax match basOperator "\v\+"
syntax match basOperator "\v\-"
syntax match basOperator "\v\*"
syntax match basOperator "\v\/"
syntax match basOperator "\v\!"
syntax match basOperator "\v\,"

hi default link basNumber   Number
hi default link basString   String
hi default link basComment  Comment
hi default link basKeyword  Keyword
hi default link basOperator Operator

let b:current_syntax = "bas"
