# Skak

## Introduktion

## Teori

### Variabler

Variabler er navne for data, der ligger i computerens RAM. Variable kan oftest ændre sig i løbet af programmets udførsel, men det behøves ikke nødvendigvis. I low-level programmeringssprog som C bliver variabler lagt på stakken. En stak er en datastruktur, hvor det seneste indsatte element er det første der kommer ud. Problemet med stakken er, at den har en størrelse på 1Mb og du skal kende størrelsen på din variabel på compile time. Forestil dig, at man vil åbne en fil og læse dens indhold. For at undgå dette problem skal man dynamisk allokere indholdet. Nedenstående pseudokode viser hvordan en fil kunne åbnes i C:

```c
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
  // Åbner filen filename.txt
  FILE *file = fopen("filename.txt", "r");

  // Finder filens størrelse i bytes.
  fseek(file, 0, SEEK_END);
  size_t size = ftell(file);
  fseek(file, 0, SEEK_SET);

  // Allokerer en buffer, der er stor nok til at indeholde filens indhold.
  char *contents = malloc(sizeof(char) * size + 1);
  memset(contents, 0, size + 1);

  // Læser filens indhold i bufferen
  fread(contents, sizeof(char), size + 1, file);

  // Lukker filen for den skal ikke bruges mere.
  fclose(file);
}
```

I high-level sprog, som JavaScript eller Python, håndteres den dynamiske allokering af en garbage collector, der deallokerer variablerne, når de ikke bruges længere. Dette gør livet som programmør langt lettere, men man opgiver kontrollen, så programmer skrevet i high-level sprog har værre performance end lowlevel, hvor du har fuld kontrol over hukommelsen.

### Forgreninger

Forgreninger handler om hvordan en betingelse kan ændre hvordan et program skal køres. Den mest typiske kontrolstruktur er en hvis-betingelse.

```ts
if (true) {
    // Kode herinde køres, fordi betingelsen er sand.
}

if (false) {
    // Kode herinde køres ikke, fordi betingelsen er falsk.
}
```

### Løkker

En løkke er en kontrolstruktur, der gentager noget baseret på om en betingelse er sand eller falsk.

En while løkke køres indtil `condition` evalueres til false, mens en for løkke er lidt mere kompliceret.

En for løkke starter med at køre `statement1` en gang. Den bruges ofte for at initialisere en variabel. `statement2` bruges som en betingelse. Hvis den er sand køres koden. `statement3` bliver kørt efter, hver

```ts
while (condition) {
  // ...
}

for (statement1; statement2; statement3) {
  // ...
}
```