# AFRITALK   
Realtime instant messaging application with end to end encryption support. Users can send and/or receive 
encrypted messages. This application is powered by a system of encoding and decoding messages leveraging 
matrix based cryptography.  

## The Encoding and decoding system overview  

#### How it works(Summary).   
###### Encrypting a text message to be sent   
Text message ---> Encode the message ---> Send the encoded message and persist your encrypted copy on disk.   
###### Reading an encrypted message   
Incoming message ---> Persist the encrypted content on disk ---> Decode the encrypted text ---> Load the intended readable message.   

##### Prerequisites for the system of encoding and decoding messages   
Both the sender and the receiver of the message is required to know:  
* A specified rule of correspondence between set of symbols(such as letters of alphabet and punctuation marks from with the messages are composed) and a set of integers shared between sender and receiver.   
* A specified non-singular matrix A that will be used for encoding.   

#### How it works(System Analysis).   
A natural correspondence between the required characters and non-negative integers is provided. The default implementation here maps supported characters to the first 77 non-negative integers.   

##### 1. Encoder  
Sending a message, the natural correspondence is used by the system to generate the numerical equivalence of the message to be sent.   
The numerical equivalence of the message is coverted to a matrix, with neccessary padding added to achieve the desired number of rows and/or columns.    
We choose an encoding matrix A such that;   
- A is non-singular(to show the matrix invertibility, determinant is a non-zero value).  
- A has only integer entries.  
- Inverse of A has only integer entries.  
- A is not very large for easy computation of its inverse.   
Sender will encode the message by means of this non-singular matrix A, by premultiplying the message matrix(i.e M) by matrix A, to obtain the encoded version of the message(i.e B).   
*Here, our message matrix M has gone through a particular transformation A to arrive at its encoded form B i.e*
```
MA=B
``` 

##### 2. Decoder  
Here, the receiver receives the encrypted message B, and knows the inverse of A (the secret in this case). Decoding will be straightforward computation, where we premultiply B by inverse of A to get back to our original message M.   
We have,
```
BA<sup>-1<sup>=M
``` 
With numerical equivalent of our message, we can generate back the text based representation of our message using the natural rule of correspondece.
