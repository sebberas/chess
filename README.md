# Chess

# Building

For at bygge projektet skal [Node](https://nodejs.org) og [Rust](https://www.rust-lang.org/tools/install) være installeret på ens computer.

**Advarsel:** Det er vigtigt at både Node og Rust er tilføjet til dine miljøvariabler. Dette tillader at man blot kan skrive `node` eller `cargo` i terminalen. Ellers får man en ```'node' is not regocnized as an internal or external command.``` fejl.

Yarn installeres via NPM:

```bash
$ npm install -g yarn
```

Derefter køres følgende i root mappen:

```
yarn install
```

Til sidst køres en af følgende:

- Produktionsmiljø: `yarn build`
- Udviklingsmiljø (For at starte udviklingsserveren): `yarn start`

**Note:** Produktionsmiljøet opretter en ny mappe kaldet `build`. I den mappe skal man enten åbne index.html eller starte en lokal server i den mappe. Eksempelvis med `Live Server` udvidelsen til Visual Studio Code. 