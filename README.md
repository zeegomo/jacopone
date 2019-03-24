# jacopone
CLI tool for Jacopone encryption/decryption of files


### Jacopone
Jacopone is a block cipher designed by me, spoiler alert: **not safe**

Jacopone is based on a 4-round Feistel network with Sha3 as round function. The block size is 512 bits and the 
key length is 256 bit. The only cipher mode of operation currently supporter is CTR and the nonce is required to be 60 bytes.
A key schedule is currently under development.
  
This construction scheme, still subject to change, is based on work by M. Luby and C. Rackoff and should be, at least theoretically,
not obviously wrong.
g
### Benchmarks:
**CPU: i5-3330 (quad-core)**

| Input size | n of threads | time |
|------------|------------:|-----:|
| 1.0 MB     |1            |0.2s  |
| 1.0 MB     |2            |0.1s  |
| 1.0 MB     |4            |0.06s |
| 100 MB     |1            |19.4s |
| 100 MB     |2            |9.8s  |
| 100 MB     |4            |5.3s  |

