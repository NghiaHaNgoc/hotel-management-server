# Hotel management

## [Server link](https://hotel-management-server.fly.dev)
 - Ensure header set `"Content-Type" : "application/json"`

- NOTE:
- `gender` field only accept `male` and `female`
- `birth_day` field follow format `YYYY-MM-DD` or `YYYY/MM/DD`
- Make sure included Bearer token in header for authorization request

## General

### End point `/employee-sign-in` and `/customer-sign-in`
- Method: `POST`
- Body:
 ```json
{
   "email": "your email",
   "password": "your password"
}
```

### End point `/profile`
- Method: `GET`

