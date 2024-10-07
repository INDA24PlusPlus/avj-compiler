# Egengjorda språk

## Variabler

Definieras med ett @ direkt följt av variabelnamnet:

```ruby
@a = 1
@b = 3 + 2 # 5
@c = 5 * a + b # 10
```

## If statements

Ordet "om" (ja, detta är ett svenskt programmeringsspråk) följt av condition, följt av ett "då" med måsvingar där koden som exekveras när conditionet är uppfyllt körs, tex:

```
@a = 1

om a == 0 då {
    print "en nolla"
} annars om a == 2 {
    print "en tvåa"
} annars {
    print "någonting annat"
}

```

## Loopar

Upprepa följt av en integer följt av namnet på iterator variabeln, exempelvis:

```
@n = 4
upprepa n @i {
    print i
}
```

## Aritmetik

Aritmetik funkar som i alla andra språk, se variabler för lite exempel på det.

# BNF (WIP)

```bnf
<add> ::= <add> "+" <integer> | <integer>


<integer> ::= <integer> <digit> | <digit>


<digit> ::= [0-9]

<letter> ::= [a-z]

<name> ::= <name> <letter> | <letter>


<mult> ::= <mult> "*" <integer> | <integer> | <add>

<division> ::= <division> "/" <integer> | <integer> | <add> | <variable>

<negation> ::= <negation> "-" <integer> | <integer> | <add> | <mult>

<assignment_operator> ::= "="

<variable_assignment> ::= "@" <name>

<variable> ::= <name>

<expression> ::= <add> | <mult> | <negation> | <division>

<assignment> ::= <variable_assignment> <assignment_operator> <expression>

<conditional_operator> ::= "==" | "!=" | "<" | ">" | ">=" | "=<"

<print> ::= "print" " " (<expression> | <variable>)

<statement> ::= <expression> | <assignment> | <print>

<comparisons> ::=  <expression> <conditional_operator> <expression>
| <variable> <conditional_operator> <variable>
|  <expression> <conditional_operator> <variable>
|  <variable> <conditional_operator> <expression>

<if_statement> ::=
"om " <comparisons> "{ " <statement> " }"
| "om " <comparisons> "{ " <statement> " }" " annars " "{ " <statement> " }"
| "om " <comparisons> "{ " <statement> " }" " annars om " <comparisons> " { " <statement> " }"

<for_loop> ::= "upprepa " <expression> " " <variable_assignment> " { " <statement> " }"
```
