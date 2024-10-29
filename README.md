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

om a == 0 {
    print "en nolla"
} annars om a == 2 {
    print 2
} annars {
    print 3
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

## Prints

Print fungerar som i princip alla andra språk men med begränsningen att programmet avslutas vid ett print, ekvivalent med att returnera 0 i en C main funktion.

# Fibonacci

Fibonacci fungerar självklart inte än, men koden kommer att se ut på det här sättet:

Note: k i det här fallet måste vara något godtycligt heltal så denna kod kommer därför inte att kompilera.
Mycket inspiration har tagits från denna [lösning](https://stackoverflow.com/questions/9122277/what-is-a-non-recursive-solution-for-fibonacci-like-sequence-in-java) för en icke-rekursiv lösning av fibonacci i Python men som anpassats för mitt språk.

```
@n = k

om n == 0 {
    print 1
}

om n == 1 {
    print 3
}

@initial = 1
@second = 3
@result = 0

upprepa n @i {
    result = 3 * second - initial
    initial = second
    second = result
}

print result
```

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

# Träd

Träd kan renderas genom att bygga ast, se `src/main.rs` och sen köra `draw_tree`. För bäst tydlighet, printa både elementen i trädet och själva trädstrukturen som följande:

```rs
let ast = parse(if_tokens);
draw_tree(ast);
```
