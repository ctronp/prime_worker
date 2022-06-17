# Prime Worker

REST API that validates if values are strings.

## Usage

### endpoint: ```/primes```

payload:

```json
{
  "values": [
    "1",
    "2",
    "3",
    "1000001"
  ]
}
```

### endpoint: ```/```

works as health path. doesn't require validation

## Environment variables

- PORT: port where the app will run, default:```8080```
- MAX_LEN: maximum quantity of digits of the numbers to test, default:```200```
- SECRET_HEADER: header to validate the requests, default:```Secret```
- SECRET: secret_header value to validate the request, default:```SecretStringExample1111000011110000```

## Return

return the numbers as keys and the result as value:

(1)key: (2)value

1. an input value
2. if it's prime ("Yes", "No", "Probably")

### Example:

#### input:

The Json can have at most 20 values.

```json
{
  "values": [
    "1",
    "2",
    "3"
  ]
}
```

#### output:

```json
{
  "1": "No",
  "2": "Yes",
  "3": "Yes"
}
```

### Probably result

if the result is ```Probably``` it means that it passed 64 iterations
of Miller Rabin primality test.

The probability of the number not being a prime is $\ \frac{3}{4}^{64} = 10^{-8} $