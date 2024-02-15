## pnet
npcap dev sdk
- https://npcap.com/#download
- download sdk
- extract
two options from here:
- place sdk somewhere on your system
    - add to path OR run $env:LIB="C:\Program Files\Npcap\sdk\Lib\x64;$env:LIB" and then cargo build

or, with the correct architecture, make sure you add Packet.lib &
wpcap.lib to your
- `C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.37.32822\lib\x64\`
directory