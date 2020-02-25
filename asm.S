# Author: Luo Jia <luojia65@hust.edu.cn> 2020-02-25

# todo: check syntax

.macro REG_READ name, offset
    .section .text.__read_ ## name
    .globl .text.__read_ ## name
    __read_ ## name:
        csrrs a0, offset, x0
        ret
.endm

REG_READ(mcountinhibit, 0x320)
