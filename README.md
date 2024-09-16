<div align="center">
  <img src="https://github.com/user-attachments/assets/f7647eab-c556-402a-a1fa-44e7f9146d59" /> <br/>

  <img src="https://img.shields.io/badge/0.1.4-red?style=for-the-badge&label=Version&color=%231457b6" />
  <img src="https://img.shields.io/badge/rust-red?style=for-the-badge&label=Language&color=%231457b6" />
  <img src="https://img.shields.io/badge/mit-red?style=for-the-badge&label=License&color=%231457b6" />
</div>
<br>

## 👀 Installation
Short instruction:
1. Download the [latest release](https://github.com/mealet/mul0/releases/latest) for you'r system (linux, windows, macos)
2. Move it to any folder on you'r PC
3. Done

## 🧭 Usage
1. Open terminal in **mul0** directory and run it:
```sh
# MacOS or Linux
./mul0

# Windows
./mul0.exe
```
2. Follow instructions from _help_ message

**Basic Example:**
```bash
$> ./mul0 hash "Hello, World! 2 + 2 = 4"
Mul0 output (printed to stderr):
14401c681e601e601f380c60090018781f3820101e601c20094809000e1009000c1809000e100900112809000ea00072

$> ./mul0 dehash "14401c681e601e601f380c60090018781f3820101e601c20094809000e1009000c1809000e100900112809000ea00072"
Mul0 output (printed to stderr):
Hello, World! 2 + 2 = 4
```

## 🦀 How it works
Algorithm takes first byte and uses it as a key. Next steps are pretty simple: <br>
1. Multiply every byte on key's byte
2. Convert every byte to hexdecimal number and resize it to 4 chars
3. Put all strings together
4. Place key at the end

## **🦛 License**
Project licensed under the MIT License.
See more in the [LICENSE file](https://github.com/mealet/mul0/blob/main/LICENSE)
