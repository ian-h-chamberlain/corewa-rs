;name Stepping Stone
;author Kurt Franke
;strategy Quick scan -> Vampire (version 9h)
;assert CORESIZE == 8000
ORG     34
ADD.F   $6,     $158
MOV.I   @0,     @157
JMZ.A   *2,     *156
MOV.I   @0,     *155
DJN.B   $-4,    #40
JMP.B   $848,   #0
JMP.B   @0,     $-2281
JMP.B   @-25,   $0
DAT.F   <2667,  <5335
DAT.F   <-15,   #10
SPL.B   #0,     #10
MOV.I   @2,     >-4
MOV.I   @1,     >-5
DJN.B   $-2,    {-3
DAT.F   $-110,  $110
MOV.A   #849,   $2152
SPL.B   #0,     $0
MOV.I   $2,     >1
MOV.I   $0,     $657
MOV.I   {-1,    <-1
MOV.I   $-7,    $3656
MOV.I   {-1,    <-1
MOV.I   $-16,   $2931
MOV.I   $-16,   $2955
MOV.I   $-10,   $2802
MOV.I   $-20,   $2800
MOV.I   {-1,    <-1
MOV.I   $0,     $-7
SPL.B   @-3,    #0
MOV.I   $0,     $-4
DAT.F   $0,     $0
JMP.B   $3,     #0
JMP.B   *6,     #-12
JMP.B   *-12,   #-6
SNE.X   $4800,  $4900
SEQ.X   $4599,  $4699
MOV.AB  #3977,  $21
JMN.B   $13,    $20
SNE.X   $2796,  $2896
SEQ.X   $2595,  $2695
MOV.AB  #1877,  $17
JMN.B   $9,     $16
SNE.X   $1592,  $1692
SEQ.X   $1391,  $1491
MOV.AB  #677,   $13
JMZ.B   $-27,   $12
ADD.AB  #100,   $11
JMZ.F   $-1,    @10
SUB.BA  $9,     $-17
MOV.I   $-18,   @8
MOV.BA  $7,     $7
SUB.F   $-19,   $6
MOV.I   $-19,   @5
MOV.I   $-21,   *4
MOV.I   $0,     @0
DJN.B   $-1,    #7
JMP.B   $-38,   #-46
MOV.BA  $2,     $2
SPL.B   #0,     #-24
MOV.I   $10,    >2
JMP.B   $612,   #0
