## Descripción

Web-Phisher is a pentesting and social engineering tool that is thought and inspired as an alternative to [SET](https://github.com/trustedsec/social-engineer-toolkit)
### **Disclaimer**

This software, "Web-Phisher," has been developed solely for research and awareness purposes in the field of computer security. The use of this tool for illegal or malicious activities is strictly prohibited. The author and contributors of this software do not assume any responsibility for any misuse that may occur with this tool.

*It is the user's responsibility to ensure that the use of this tool complies with all local laws and regulations, and to obtain proper consent before conducting tests on systems or networks that do not belong to them.*
## Platforms

- Mac OS X
- Windows
- Linux
## Compilation

Rust is needed for this task

Get it from its official site:  [rust-lang.org](https://www.rust-lang.org/)

```bash
git clone https://github.com/nonskilledeveloper/web-phisher web-phisher/
cd web-phisher
sudo cargo build --release
```
## Usage example

```bash
~/devs/hacking/web-phisher-release 0.1 
ls
web-phisher
~/devs/hacking/web-phisher-release 0.1 
sudo ./web-phisher
Web Phisher - Loading ...

🥚 🐣 🐥 🐓

By nonskilledeveloper

You can clone one of the following sites:
-------------------------------------------
1) Facebook        | 2) Gmail  | 3) Tiktok |
4) Facebook Mobile | 5) Soon...|           |
-------------------------------------------
1
Cloning URL: https://web.facebook.com/login
Site cloned successfully
Adding Hijaker Script...
Site is now poisoned!
Starting Server...
Server running on: http://192.168.101.6:80/
```
![server running](https://github.com/nonskilledeveloper/web-phisher/blob/master/IMG/Pasted%20image%2020230917232850.png)

This is how it looks when the credentials are captured

```bash
Data Found: Json: Credentials { email: "nonskilledev@hackermail.com", pass: "h4ck3rpass#" }
```
## Pugs?

![pug](https://github.com/nonskilledeveloper/web-phisher/blob/master/IMG/Pug1-modified.png)

If you find any bug or pug, feel free to report it [here](https://github.com/nonskilledeveloper/web-phisher/issues) 
