;redcode quiet
;name Fire Storm v1.1
;author W. Mintardjo
;strategy Stone with anti-imp core clear
;assert CORESIZE==8000
ORG     82
SPL.B   $0,     $1
JMZ.B   $-1,    $1
JMP.B   @-1,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
DAT.F   #2,     #1
DAT.F   #5,     #1
JMN.B   $-2,    $1
SPL.B   $-1,    $1
SPL.B   $0,     <1
SPL.B   $-1,    @1
JMZ.B   $-5,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
JMN.B   $-2,    $1
JMZ.B   $-5,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
SPL.B   $0,     $1
JMZ.B   $-1,    $1
JMP.B   @-1,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
JMN.B   $-2,    $1
SPL.B   $-1,    $1
SPL.B   $0,     <1
SPL.B   $-1,    @1
JMZ.B   $-5,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
JMN.B   $-2,    $1
JMZ.B   $-5,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
SPL.B   $0,     $1
JMZ.B   $-1,    $1
JMP.B   @-1,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
JMN.B   $-2,    $1
SPL.B   $-1,    $1
SPL.B   $0,     <1
SPL.B   $-1,    @1
JMZ.B   $-5,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
JMN.B   $-2,    $1
JMZ.B   $-5,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
SPL.B   $0,     $1
JMZ.B   $-1,    $1
JMP.B   @-1,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
JMN.B   $-2,    $1
SPL.B   $-1,    $1
SPL.B   $0,     <1
SPL.B   $-1,    @1
JMZ.B   $-5,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
JMN.B   $-2,    $1
JMZ.B   $-5,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
SPL.B   $0,     $1
JMZ.B   $-1,    $1
JMP.B   @-1,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
JMN.B   $-2,    $1
SPL.B   $-1,    $1
SPL.B   $0,     <1
SPL.B   $-1,    @1
JMZ.B   $-5,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
JMN.B   $-2,    $1
JMZ.B   $-5,    $1
DAT.F   #2,     #1
DAT.F   #5,     #1
MOV.I   $14,    $4014
MOV.I   $12,    <-1
MOV.I   $10,    <-2
MOV.I   $8,     <-3
MOV.I   $6,     <-4
MOV.I   $4,     <-5
MOV.I   $9,     $4022
MOV.I   $9,     $3986
JMP.B   $4002,  <2000
SPL.B   $0,     <-17
MOV.I   <-154,  $157
SUB.F   $17,    $-1
JMP.B   $-2,    <-2000
MOV.I   @-3,    <15
DJN.B   $-1,    <3975
DAT.F   #155,   #-155
DAT.F   <-18,   #0