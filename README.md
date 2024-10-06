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
} annars om == 2 {
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
