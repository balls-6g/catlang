# The cli for the catc...

## compile

compiling the file:
```bash
catc app.cat
```

to a file:
```bash
catc app.cat -o app
```

with some library:
```bash
catc app.cat -o app -l ~/.cat/lib/catui
```

and some compiler extensions:
```bash
catc app.cat -o app -l ~/.cat/lib/catui/ -e ~/.cat/lib/ext/nom
```

## compile to cat-IR

compile:
```bash
catc ir app.cat
```

to a file:
```bash
catc ir app.cat -o app.ctir
```

with some library:
```bash
catc ir app.cat -o app.ctir -l ~/.cat/lib/catui
```

and some compiler extensions:
```bash
catc ir app.cat -o app.ctir -l ~/.cat/lib/catui/ -e ~/.cat/lib/ext/nom
```

## from cat IR into binary

```bash
catc irc app.ctir
```

to a file:
```bash
catc irc app.ctir -o app
```

with some library:
```bash
catc irc app.ctir -o app -l ~/.cat/lib/catui
```

and some compiler extensions:
```bash
catc irc app.ctir -o app -l ~/.cat/lib/catui/ -e ~/.cat/lib/ext/nom
```
