# Qochel - Pure C, No Extras

English Translation is [here](readme_en.md).  

> [!NOTE] Qochel v0.1.0 has been released!
> `v0.1.0` is the first stable version of Qochel.  
> 👉 For more information, see Release page.

## Overview

`Qochel`はRustで書かれた音楽へのリスペクトです。  
C言語をケッヘル番号風のQochel番号によって難読化できる文法を持っています。  

## Installation

### Build from source

Cargoを使用してビルドします。

```terminal
gh repo clone hthcrwzy/Qochel
cd qochel
cargo build
```

生成された`target`ディレクトリ内の実行ファイルを適当な場所に配置してください。

## Translation Table

キーワード、変数名、文字列は散逸を防ぐためQochel（コッヘル）番号で管理されます。

### C Keywords

| C Keyword        | Qochel Number |
| :--------------- | :------------ |
| `auto`           | `Q1`          |
| `break`          | `Q2`          |
| `case`           | `Q3`          |
| `char`           | `Q4`          |
| `const`          | `Q5`          |
| `continue`       | `Q6`          |
| `default`        | `Q7`          |
| `do`             | `Q8`          |
| `double`         | `Q9`          |
| `else`           | `Q10`         |
| `enum`           | `Q11`         |
| `extern`         | `Q12`         |
| `float`          | `Q13`         |
| `for`            | `Q14`         |
| `goto`           | `Q15`         |
| `if`             | `Q16`         |
| `inline`         | `Q16a`        |
| `int`            | `Q17`         |
| `long`           | `Q18`         |
| `register`       | `Q19`         |
| `restrict`       | `Q19a`        |
| `return`         | `Q20`         |
| `short`          | `Q21`         |
| `signed`         | `Q22`         |
| `sizeof`         | `Q23`         |
| `static`         | `Q24`         |
| `struct`         | `Q25`         |
| `switch`         | `Q26`         |
| `typeof`         | `Q27`         |
| `union`          | `Q28`         |
| `unsigned`       | `Q29`         |
| `void`           | `Q30`         |
| `volatile`       | `Q31`         |
| `while`          | `Q32`         |
| `_Alignas`       | `Q32a`        |
| `_Alignof`       | `Q32b`        |
| `_Atomic`        | `Q32c`        |
| `_Bool`          | `Q33`         |
| `_Complex`       | `Q34`         |
| `_Generic`       | `Q34a`        |
| `_Imaginary`     | `Q35`         |
| `_Noreturn`      | `Q36`         |
| `_Static_assert` | `Q37`         |
| `_Thread_local`  | `Q38`         |

### Preprocessor Keywords

前処理司令はコッヘル前処理指令番号（QP）で管理されます。

| Preprocessor Keyword | Qochel Preprocessor Number |
| :------------------- | :------------------------- |
| `#if`                | `QP1`                      |
| `#elif`              | `QP2`                      |
| `#else`              | `QP3`                      |
| `#defined`           | `QP4`                      |
| `#ifdef`             | `QP5`                      |
| `#ifndef`            | `QP6`                      |
| `#define`            | `QP7`                      |
| `#undef`             | `QP8`                      |
| `#include`           | `QP9`                      |
| `#line`              | `QP10`                     |
| `#error`             | `QP11`                     |
| `#pragma`            | `QP12`                     |
| `_Pragma`            | `QP13`                     |

### Extra Keywords

拡張キーワードはコッヘル拡張キーワード番号（QEK）で管理されます。

| Extra Keywords | Qochel Extra Keyword Number |
| :------------- | :-------------------------- |
| `asm`          | `QEK1`                      |
| `fortran`      | `QEK2`                      |

### Custom Keywords

変数名や構造体名、関数名といったものはにQochel Anhang（QAnh）を記述し使用します。  
`QAhn.toml`:

```TOML
QAhn1 = "Mozart"
QAhn2 = "Haydn"
QAhn3 = "Beethoven"
QAhn4 = "Debussy"
QAhn5 = "Dvořák" # Clang allows this.
QAhn6 = "瀧廉太郎" # Clang allows this too.
```

### Strings

> [!IMPORTANT]
> 正しい絵文字が表示されない場合があります。
> 
文字列はTOMLにQuartet（Qua）を記述し使用します。  
`Quartet.toml`:

```TOML
Qua1 = "1st violin"
Qua1a = "2nd violin"
Qua2 = "Viola"
Qua3 = "Cello"
Qua4 = "Piano" # Don't have to be "Quartet"!
```

## Example

最終的なディレクトリ構成:

```tree
.
├── QAhn.toml     <--- Configuration file
├── Quartet.toml  <--- Configuration file
├── runner        <--- Executable file
├── main.c        <--- Executable file
└── source.qochel <--- Source file
```

`QAhn.toml`:

```TOML
QAhn1 = "<stdio.h>" # You can replace header file name.
QAhn1a = "printf" # You can use (an) alphabet(s).
QAhn1aa = "Qochel Qochel Qochel"
QAhn1b = "Halo" # You don't have to use.
QAhn2 = "greeting"
QAhn3 = "main" # You can write names of functions.
```

`Quartet.toml`

```toml
Qua1 = "Hello, world!"
Qua2 = "%s\n" # Formatting string is ok
```

`source.qochel`:

```c
// You can write comments using // or /**/
QP9 QAhn1 // Qochel numbers must be separated by spaces
Q17 QAhn3(Q30) {
    Q5 Q4* QAhn2 = Qua1;
    QAhn1a(Qua2, QAhn2);
    Q20 0;
}
```

run:

```terminal
qochel source.qochel
```

QochelはCコードを生成し、`clang`でビルドします。

```c
#include <stdio.h>

int main(void) {
    const char* greeting = "Hello, world!";
    printf("%s\n", greeting);
    return 0;
}
```

run:

```terminal
$ ./runner
Hello, world!
```

## System

1. `Bach`がソースコードや設定ファイルを読み込みます。
2. `Haydn`がソースファイルをCコードに変換します。
3. `Mozart`ががCコードをビルドして成果物を保存します。

## License

QochelはApache License, Version 2.0の下で利用可能です。  
詳細は[LICENSE](LICENSE)を御覧ください。

```txt
Copyright 2024 hthcrwzy

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
