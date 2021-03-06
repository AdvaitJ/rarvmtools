#include <constants.rh>
#include <crctools.rh>
#include <math.rh>
#include <util.rh>
; vim: syntax=fasm

_start:
    push    #8
    push    #0x12345678
    call    $_find_vector_x
    cmp     r0, #0x6a0720dd
    jz      $next
    call    $_error
next:
    push    #7
    push    r0
    call    $_find_vector_x
    cmp     r0, #0x60e880da
    jz      $finish
    call    $_error
finish:
    mov     [#0x1000], #0x000a4b4f
    mov     [VMADDR_NEWBLOCKPOS],  #0x1000   ; Pointer
    mov     [VMADDR_NEWBLOCKSIZE], #3        ; Size
    call    $_success
