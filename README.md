# jacopone
CLI tool for Jacopone encryption/decryption of files


### Jacopone
Jacopone is a block cipher designed by me, spoiler alert: **not safe**

Jacopone is based on a 4-round Feistel network with Sha3 as round function. The block size is 256 bits and the 
key length is 256 bit. The only cipher mode of operation currently supporter is CTR and the nonce is required to be 60 bytes.
A key schedule is currently under development.
  
This construction scheme, still subject to change, is based on work by M. Luby and C. Rackoff and should be, at least theoretically,
not obviously wrong.



