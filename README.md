This project is a PoC of using [calamine](https://docs.rs/calamine/latest/calamine/) crate to parse excel files in wasm.

Calamine currently relies much on `std::io::fs`, which is not supported when compiling to wasm.

I uses an [unmerged PR of calamine](https://github.com/tafia/calamine/pull/256) to do this PoC (Appreciate the work of Hanif Ariffin!), and I would switch to official calamine library if this PR is merged.

Before that the wasm have to be compiled with a feature branch, please follow the below to build, or use `wasm` file directly.

# Play with it
```bash
# Launch a server with whatever tool you like in the root path, then open index.html in your browser
npx http-server ./
```
### What does it do?
Read an excel file and parse it to json string. The json object is a 2d array of strings, empty cells are translated to empty strings.
For example:
### Excel File
|   A    |   B    |
|  ----  |  ----  |
| Cell_1 | Cell_2 |
| Cell_3 | Cell_4 |
| Cell_5 |        |

### Output result:
```json
[["Cell_1","Cell_2"],["Cell_3","Cell_4"],["Cell_5",""]]
```

# Build
```bash
# Checkout Hanif Ariffin's feature branch
git clone git@github.com:hbina/calamine.git
cd calamine && git checkout hbina-add-ability-to-open-workbook-from-byes && cd ..
# Build wasm
wasm-pack build --target web
```
