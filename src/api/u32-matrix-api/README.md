# u32-matrix-api

The API which extends the matrix-synapse API's functionality.
This API's only external dependency is a Matrix-Synapse API.

# Cryptography

The API has a custom way of managing accounts and sessions via API keys which is entirely stateless.
The API key equation may be expressed as follows:

Let `E` be some encryption function.<br>
Let `k` be the resulting api key.<br>
Let `u` be a string username.<br>
Let `p` be a string password.<br>
Let `s` be a secret known only by the instance of **u32-matrix-api**.<br>

Then, `k = E( u | p , s)`.